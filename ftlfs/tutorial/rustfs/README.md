# rustfs
## 环境搭建
### rust版本
开发使用的`Rust`版本如下，没有测试运行本项目需要的最低版本，如出现编译失败等问题，可尝试更新至该版本：
```bash
rustc 1.67.0-nightly (7632db0e8 2022-12-08) 
```
### 安装fuse
本项目基于`fuse`开发，所以需要安装`fuse`以及`libfuse-dev`，以ubuntu为例：
```bash
sudo apt-get update 
sudo apt install fuse libfuse-dev   
```
## 运行
首先切换到项目的`tests`目录下:
```bash
cd rustfs/tests/
```
将`rustfs`挂载到`tests/mnt`目录下：
```bash
make mount
```
此时可以在`tests/mnt`目录下执行`ls`,`touch`,`mkdir`等命令，对`rustfs`进行操作。

执行`make umount`可卸载`rustfs`，执行`make clean`可清除`rustfs`上次挂载的数据，如不执行`make clean`，则下次挂载时会读取上次挂载的数据。
## 测试
运行所有单元测试：
```bash
make unit_test
```
## 其他命令
```bash
make mount_mt # 多线程挂载
make unit_test_debug # 运行所有单元测试并打印日志
make ddriver_test # 驱动封装层测试
make replacer_test # 替换算法测试
make buffer_test # 缓存层测试
make loop_buffer_test # 循环测试缓存层100次（用于测试缓存层的线程安全性）
make fs_test # 文件系统层测试
```