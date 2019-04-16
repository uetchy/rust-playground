# Rust+WASM Playground

## Prerequisite

- rustup target add wasm32-unknown-unknown
- `npm i -g serve`

## Build

```
make
serve
```

or

```
make start
```

## Tips

http://nmi.jp/2018-03-19-WebAssembly-with-Rust

`#[no_mangle]` と `pub extern "C"` によって、この関数がマングリングされずに出力されるようになります。

```
WebAssembly.instantiateStreaming(fetch(...))
  .then(wasmModule => {})
```

Chrome の場合は`#instantiateStreaming`が使える。

`wasm-bindgen`で JS と WASM 間のデータバインディングを行う。
