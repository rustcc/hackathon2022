extern crate embed_resource;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    // 只有在Windows版本才能设置可执行文件的图标
    if target.contains("windows") {
        embed_resource::compile("build/windows/icon.rc");
    }
}
