mkdir -p pkg
PATH=$PATH:$(dirname $(find $(rustc --print sysroot) -name 'rust-lld')) \
RUSTFLAGS='-C target-feature=+atomics,+bulk-memory' rustup run nightly wasm-pack build -t no-modules -- -Z build-std

python3 -m http.server