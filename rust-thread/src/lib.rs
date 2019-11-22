#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use wasm_bindgen::prelude::*;

mod thread;

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
pub fn start() -> Result<(), JsValue> {
  let thread = thread::WorkerThread::new("./worker.js", "worker1")?;
  thread.run(move || {
    console_log!("run in wasm");
  })?;

  Ok(())
}