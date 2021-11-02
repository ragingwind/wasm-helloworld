import('../wasm/add.rs').then(({add}) => {
  document.body.innerHTML += `add(2, 3) = ${add(2, 3)}<br/>`
})

import('../wasm/square.wasm').then(({square}) => {
  document.body.innerHTML += `square(10) = ${square(10)}<br/>`
})