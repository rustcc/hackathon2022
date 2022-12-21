# [my_embassy](https://github.com/overheat/my_embassy)
## 大使带你~异步~一步上云

> Embassy是嵌入式MCU的async异步框架。

![图标](https://iosoftblog.files.wordpress.com/2022/11/pico_w2.jpg)
> 开源硬件Raspberry Pi Pico W

1. Raspberry Pi Pico W, 启动 TCP server 监听 1234 端口
2. Linux PC, `nc <ip-address> <port>`, 并发送任意字符
3. Raspberry Pi Pico W, 启动内部温度传感器
4. Raspberry Pi Pico W, 利用自身Wi-Fi上传传感器数据到云端Cloud
5. PC 可以用websocket等协议监听云端数据

> 异步IO框架Embassy使嵌入式编程更简洁，2022年底embedded-hal-async/embedded-nal-async等trait即将稳定，期待2023有完善的网络协议栈实现。
可以带来快速启动，成本低廉，易于维护等好处。


