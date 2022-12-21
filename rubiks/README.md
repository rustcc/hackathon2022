# Rubik's cube Simulator and Solver


[WASM Preview](https://user-images.githubusercontent.com/217027/208389491-66023580-3cce-4abf-9b32-879a5db7ee34.mp4)


## Feature list

- [x] 显示魔方(NxN)
- [x] 旋转魔方
  - [x] 鼠标旋转
  - [x] 按键旋转(U, D, F, B, L, R)
- [x] 练习模式
- [x] 计时模式
- [x] wasm
- [x] 3x3求解

## 如何运行

### 原生平台

```bash
cargo run
```

### 网页wasm

需要安装trunk `cargo install --locked trunk`

然后运行 `trunk serve`, 打开http://127.0.0.1:8080

## References

- https://ruwix.com/
- https://worldcubeassociation.org/regulations/#article-12-notation
- https://www.jaapsch.net/puzzles/thistle.htm