# Hello World for WebAssembly

> The first step for beginners about WASM, WebAssembly

# Prerequisites

WebAssembly is required bunch of preperations for compiling native soruce codes. While you use emscripten directly install on your system, Surmma at Google Chrome Team suggests [the alternative way](https://developers.google.com/web/updates/2018/03/emscripting-a-c-library) with docker. 

## Build Environment with Docker

## Build Environment with Emscripten Toolchain

- Setup git, CMake, gcc and python on your system
- Install [Emscripten toolchain](http://webassembly.org/getting-started/developers-guide/) and [The WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt)
- Install compilers for Go, Rust and C/C++

# Installation Dependencies

Each demo has own npm dependencies or own environment to build and start application. In case of Node.js environment, for getting convenience of installation, you should install dependencies with `yarn install --modules-folder=../node_modules` 

# Demos

## C/C++ Languages
- clang: Emscripten and WASM APIs, Basic sample, show two of types compiling, static and dynamic
- clang-thread: Show how to use c with thread. Implementation of official sample from Google\
- clang-array: Show how to manage numberic array
- clang-simd: Show how WASM is working with SIMD

## Rust Languages
- rust: Rust Language with Emscripten and WASM APIs
- rust-parcel: Rust Language with parcel bundler
- rust-thread
- rust-wasm-bindgen: Rust Language with wasm-bindgen
- rust-wasm-pack
- rust-webgl

## Typescript
- ts-assembly-script: Typescript(AssemblyScript) with WASM APIs

## Golang
- go: Go with WASM

## asm.js
- asm-webpack: asm.js with WABT and webpack 4

# License

MIT @ Jimmy Moon
