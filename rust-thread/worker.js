importScripts("./pkg/rust_thread.js");

console.log('worker: worker started', self.name);

self.onmessage = event => {
  // initialize with shared module and memoty from main thread
  const initialized = wasm_bindgen(...event.data)
    .catch(err => {
      setTimeout(() => {
        throw err;
      })

      throw err;
    });

  self.onmessage = async event => {
    await initialized;
    try {
      wasm_bindgen.child_entry_point(event.data);
    } catch (err) {
      console.log(err);
    }
  }
}