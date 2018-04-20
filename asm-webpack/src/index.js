import('../wasm/fibonacci.wasm').then(m => {
  document.write(m.fibonacci(10))
})