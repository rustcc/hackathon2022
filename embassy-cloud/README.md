## 基于 开源硬件[Raspberry Pi Pico W](https://www.raspberrypi.com/products/raspberry-pi-pico/)和[Embassy](https://github.com/embassy-rs/embassy)嵌入式(no_std) 异步框架 的硬件项目

![图标](https://iosoftblog.files.wordpress.com/2022/11/pico_w2.jpg)
> 开源硬件Raspberry Pi Pico W

### Getting start
0. `git clone https://github.com/overheat/my_embassy`
1. Raspberry Pi Pico W, 启动 TCP server 监听 1234 端口🕸️
2. Linux PC, `nc <ip-address> <port>`, 并发送任意字符🔤
3. Raspberry Pi Pico W, 启动内部温度传感器🌡️
4. Raspberry Pi Pico W, 利用自身Wi-Fi上传传感器数据到云端Cloud(drogue.io)☁️
5. PC 可以用websocket等协议监听云端数据👂

### Rust技术栈与embedded async生态贡献
|  Layer   | Repo  |  PR   | Issue  |
|  ----  | ----  |  ----  | ----  |
| app  | drogue |   | 1 |
| runtime  | embassy | 2  | 4 |
| TCP/IP  | smoltcp | 1  |  |
| 物理层  | pico | 1  | 1 |
| debug  | probe |   | 3 |


### 总结
异步IO框架Embassy使嵌入式编程更简洁，2022年底embedded-hal-async/embedded-nal-async等trait即将稳定，期待2023有完善的网络协议栈实现。
可以带来快速启动，成本低廉，易于维护等好处。

### Todo:
- 增加关键数据个人云存储功能，比如加密货币的token/密码/日记/语音等
- 在Raspberry Pi Pico W上启动DNS服务，利用云连接做到类似httpsDNS的个人工具
- 在Raspberry Pi Pico W上启动NTP服务，甚至是最新的PTP（us级别的）时间同步
- 其他？GPG？

### 附录
1. Rust embedded 生态

|  维护者   | 名称  |  描述   | 
|  ----  | ----  |  ----  | 
| *Drogue  | [embedded-tls](https://github.com/drogue-iot/embedded-tls) |  a Rust-native TLS 1.3 implementation | 
| Drogue  | [embedded-update](https://github.com/drogue-iot/embedded-update) | firmware update protocol  | 
| *Drogue  | [reqwless](https://github.com/drogue-iot/reqwless) |  HTTP client | 
| [Embassy](https://github.com/embassy-rs)  | embedded-io | IO traits for embedded systems.  |  
| *[Rust Embedded Community](https://github.com/rust-embedded-community)  | embedded-nal | An Embedded Network Abstraction Layer | 
| *Smoltcp  | [smoltcp](https://github.com/smoltcp-rs/smoltcp) | a standalone, event-driven TCP/IP stack | 
| [Rust Embedded Community](https://github.com/rust-embedded-community)    | embedded-storage | An Embedded Storage Abstraction Layer | 
| [Rust Embedded Community](https://github.com/rust-embedded-community)   | embedded-sdmmc-rs | A SD/MMC library with FAT16/FAT32 support, suitable for Embedded Rust systems | 
| [Rust Embedded](https://github.com/rust-embedded)  | embedded-dma | DMA  | 
| [Rust Embedded](https://github.com/rust-embedded)  | embedded-alloc |  A heap allocator for Cortex-M processors | 
| [Rust Embedded](https://github.com/rust-embedded)  | embedded-hal | A Hardware Abstraction Layer (HAL) for embedded systems  |  

`* Networking 相关`


