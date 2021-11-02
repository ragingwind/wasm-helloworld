# rust-wasi-node example

> The demo shows how WASI work with Node.js

# How to make

## Install cargo-wasi

```
cargo install cargo-wasi
```

## Create a project as lib

```
cargo new rust-wasi-node --lib
```

## Update Cargo.toml for lib project

add `[lib]` section below

```
[lib]
crate-type = ['cdylib']
```

## Implement `src/lib.rs` 

Write(Copy) code with below into `src/lib.rs`

```
use std::io::prelude::*;
use std::fs;

// export name with no mangling
#[no_mangle]
pub extern "C" fn print_hello() {
    // use system stdio
    println!("Hello, world!");

    // use system file io
    let mut file = fs::File::create("/host/helloworld.txt").unwrap();

    // write the text to the file we created
    write!(file, "Hello world!\n").unwrap();
}
```

## Build

Run commands below

```
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
```

## Run with wasmtime

Pass `/host` path to map host device location to virtual filesystem. To current directory(.) on the host filesystem will be mapped to `/host` virtual filesystem. For example:

```
--mapdir /host::.
```

Run wasmtime with using commadn below

```
wasmtime --mapdir /host::. --invoke print_hello target/wasm32-wasi/debug/rust_wasi_node.wasm
cat ./helloworld.txt
```

## Run on Node.js

Create a file to `runtime/index.mjs`

```
mkdir -p runtime
touch runtime/index.mjs
```

Write this code to `runtime/index.mjs`:

```
'use strict';

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { WASI } from 'wasi';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const wasi = new WASI({
  // Same as --mapdir of wasmtime, map virtual filesystem to host filesystem
  preopens: {
    '/host': process.cwd(),
  },
});

// pass import Object to WASM to use host APIs
const importObject = { wasi_snapshot_preview1: wasi.wasiImport };

// Load, and compile, and instantiate WASM
const wasm = await WebAssembly.compile(fs.readFileSync(path.resolve(__dirname, '../target/wasm32-wasi/debug/rust_wasi_node.wasm')));
const instance = await WebAssembly.instantiate(wasm, importObject);

// WASI try to initialize WASM instanced
wasi.initialize(instance);

// Run WASM function
instance.exports.print_hello();
```

Before running, please make sure that the version of Node.js. It have to be over 16

```
node -v
```

And then run with experimental flag

```
node --experimental-wasi-unstable-preview1 ./runtime/index.mjs
cat ./helloworld.txt
```

# Reference

- https://github.com/bytecodealliance/wasmtime/blob/main/docs/wasm-rust.md#writing-libraries
- https://github.com/bytecodealliance/wasmtime/blob/main/docs/wasm-rust.md#webassembly-interface-types
- https://wasmbyexample.dev/examples/wasi-hello-world/wasi-hello-world.rust.en-us.html
- https://nodejs.org/api/wasi.html