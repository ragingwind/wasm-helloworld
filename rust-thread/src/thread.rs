use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Worker, WorkerOptions};

pub struct WorkerThread {
  worker: Rc<Worker>
}

pub struct Work {
  pub func: Box<dyn FnOnce() + Send>,
}

impl WorkerThread {
  pub fn new(script: &str, name: &str) -> Result<WorkerThread, JsValue> {
    let mut options = WorkerOptions::new();
    options.name(String::from(name).as_str());

    let worker = Worker::new_with_options(script, &options)?;
    let thread = WorkerThread {
      worker: Rc::new(worker)
    };
    
    let wasm = js_sys::Array::new();
    wasm.push(&wasm_bindgen::module());
    wasm.push(&wasm_bindgen::memory());
    thread.post_message("init", &wasm)?;

    Ok(thread)
  }

  pub fn post_message(&self, command: &str, payload: &JsValue) -> Result<(), JsValue> {
    let commands = js_sys::Array::new();
    commands.push(&JsValue::from_str(command));
    commands.push(payload);
    self.worker.post_message(&commands)?;
    Ok(())
  }

  pub fn run(&self, f: impl FnOnce() + Send + 'static) -> Result<(), JsValue> {
    let work = Box::new(Work { func: Box::new(f) });
    let ptr = Box::into_raw(work);

    match self.post_message("work", &JsValue::from(ptr as u32)) {
      Ok(()) => Ok(()),
      Err(e) => {
        unsafe {
          drop(Box::from_raw(ptr));
        }
        Err(e)
      }
    }
  }
}

#[wasm_bindgen]
pub fn thread_execute(ptr: u32) -> Result<(), JsValue> {
    let ptr = unsafe { Box::from_raw(ptr as *mut Work) };
    (ptr.func)();
    Ok(())
}