function setEnvironment({env = {}}) {
  if (!env.memory) {
    env.memory = new WebAssembly.Memory({
      initial: 256,
      maximum: 256
    })
  }
  
  if (!env.table) {
    env.table = new WebAssembly.Table({
      initial: 0,
      maximum: 0,
      element: 'anyfunc'
    })
  }
  
  return {
    ...{
      tableBase: 0,
      memoryBase: 0,
      STACKTOP: 0,
      STACK_MAX: env.memory.buffer.byteLength,
      abortStackOverflow() {},
      abort() {}
    },
    ...env
  }
}

async function wasmInstantiate(req, importObject = {}) {
  const bytes = await window.fetch(req).then(x => x.arrayBuffer())

  importObject = {
    global: {},
    ...importObject,
    env: setEnvironment(importObject)
  }
  
  return {
    ...await WebAssembly.instantiate(bytes, importObject),
    ...importObject
  }
}