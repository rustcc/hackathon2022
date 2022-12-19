`rustfs`采用了自底向上的分层设计：

![rustfs](pics/img.png)

我们将从`driver`层开始，一步步实现`rustfs`的功能。

### 虚拟设备驱动接口

虚拟设备驱动是一个由C语言编写的，模拟设备驱动程序行为的静态链接库，位于`lib`目录下。`rustfs`通过调用`driver`层提供的接口来与虚拟设备驱动进行交互。

之所以使用C语言编写的虚拟设备驱动，是为了模拟真实的开发情况，虽然Rust语言可以用来编写设备驱动程序，但是仍然有大量的设备驱动程序是用C语言编写的。我们正好借此学习如何在Rust语言中调用C语言编写的二进制库。事实上这相当简单。

### FFI

在项目的根目录下有一个`build.rs`脚本，它会在编译`rustfs`之前执行。我们可以在`build.rs`中添加代码链接`lib`目录下的虚拟设备驱动程序。

`build.rs`中已经提供了一个链接的示例：

```rust
fn main() {
    //TODO - add your code here
    println!("cargo:rustc-link-lib=fuse");
}
```

这里链接了`libfuse`库，我们可以模仿这个例子，链接`lib`目录下的虚拟设备驱动程序：

```Rust
fn main() {
    println!("cargo:rustc-link-lib=fuse");
    println!("cargo:rustc-link-lib=ddriver");
}
```

运行`ddriver`测试：

```bash
cd tests
make ddriver_test 
```

可以看到编译失败，出现如下报错：

```bash
note: /usr/bin/ld: cannot find -lddriver
```

原因是链接器的默认搜索路径中没有`lib`目录，我们需要在`build.rs`中添加搜索路径：

```rust
fn main() {
    println!("cargo:rustc-link-lib=fuse");
    println!("cargo:rustc-link-lib=ddriver");
    println!("cargo:rustc-link-search=lib");
}
```

再次运行`ddriver`测试，可以看到编译成功。但是目前还无法通过测试。

## 封装磁盘层接口

TODO: 删除部分代码，设计填空

## 并发控制

注意我们封装的接口`read_page`和`write_page`都是先调用`seek`定位到指定的页，然后再读写。如果一个线程`seek`之后，还没有来得及读写，被另一个线程抢占了CPU，那么这个线程可能读写错误的页。

这个问题不是本节的重点，也不会影响本节的测试，我们会在后面的章节中解决这个问题。