importScripts("./pkg/rust_thread.js");

console.log('worker: worker started', self.name);

self.onmessage = async ({data}) => {
  console.log('message from wasm', data);
  try {
    switch (data[0]) {
      case 'init': 
        await wasm_bindgen(...data[1]);
        break;
      case 'work':
        wasm_bindgen.thread_execute(data[1]);
        break;
    }
  } catch (e) {
    console.error(e);
  }
}