# Prepare


```
rustup target add wasm32-unknown-unknown
```

```
cargo install -f wasm-bindgen-cli
```


# Package
```
cargo build --release --target wasm32-unknown-unknown
```


```
cp ./assets/* -r ./out/

wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/maze_of_ferris.wasm

```

# Play online

