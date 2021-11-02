mkdir -p pkg
rustc --target wasm32-unknown-unknown -O ./src/lib.rs -o ./pkg/square.wasm
