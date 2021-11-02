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