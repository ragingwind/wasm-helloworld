# Rust with wasm-bindgen 

> This sample has been wrote following by [Basic Usage](https://github.com/rustwasm/wasm-bindgen/blob/master/guide/src/basic-usage.md) with wasm-bindgen. Webpack will served files because of a [lack of features in parcel](https://github.com/rustwasm/wasm-bindgen/issues/182)

## Changes

- Using `import()` for asynchrounos loading
- Using npm script instead of `build.sh`
- Custom path with Cargo

## Uses

```sh
# Install deps
yarn install

# Build wasm source first. Generated files will be placed at 'src'
yarn cargo

# Start dev server of webpack than go to the local server
yarn start
```

# License

MIT @ CODEBUSKING