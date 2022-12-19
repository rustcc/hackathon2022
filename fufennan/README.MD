# 准备工作


```
rustup target add wasm32-unknown-unknown
```

```
cargo install -f wasm-bindgen-cli
```


# 打包
```
cargo build --release --target wasm32-unknown-unknown
```


```

wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/maze_of_ferris.wasm

```

# 在线游玩

[点我](https://willser.github.io/maze_of_ferris.html)
