#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};
use web_sys::{ErrorEvent, Event, Worker, WorkerOptions};

macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
}

#[wasm_bindgen]
pub struct WorkerPool {
  state: Rc<PoolState>,
}

struct PoolState {
  workers: RefCell<Vec<Worker>>,
}

struct Work {
  func: Box<dyn FnOnce() + Send>,
}

#[wasm_bindgen]
impl WorkerPool {
  #[wasm_bindgen(constructor)]
  pub fn new(initial: usize) -> Result<WorkerPool, JsValue> {
    let pool = WorkerPool {
      state: Rc::new(PoolState {
        workers: RefCell::new(Vec::with_capacity(initial)),
      }),
    };

    for id in 0..initial {
      let worker = pool.spawn(id).unwrap();
      pool.state.workers.borrow_mut().push(worker);
    }

    Ok(pool)
  }

  fn spawn(&self, id: usize) -> Result<Worker, JsValue> {
    let mut workerName = String::from("worker");
    workerName.push_str(&id.to_string());

    let mut workerOptions = WorkerOptions::new();
    workerOptions.name(workerName.as_str());

    let worker = Worker::new_with_options("./worker.js", &workerOptions)?;

    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::module());
    array.push(&wasm_bindgen::memory());
    worker.post_message(&array)?;

    Ok(worker)
  }

  fn execute(&self, f: impl FnOnce() + Send + 'static) -> Result<Worker, JsValue> {
    let worker = match self.state.workers.borrow_mut().pop() {
      Some(worker) => Ok(worker),
      None => self.spawn(0),
    }?;

    let work = Box::new(Work { func: Box::new(f) });
    let ptr = Box::into_raw(work);

    match worker.post_message(&JsValue::from(ptr as u32)) {
      Ok(()) => Ok(worker),
      Err(e) => {
        unsafe {
          drop(Box::from_raw(ptr));
        }
        Err(e)
      }
    }
  }

  #[wasm_bindgen]
  pub fn start(&self, count: usize) {
    for _ in 0..count {
      let worker = self.execute(move || {
        console_log!("run in wasm");
      });
    }
  }
}

#[wasm_bindgen]
pub fn child_entry_point(ptr: u32) -> Result<(), JsValue> {
    let ptr = unsafe { Box::from_raw(ptr as *mut Work) };
    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    (ptr.func)();
    global.post_message(&JsValue::undefined())?;
    Ok(())
}