#![feature(raw_ref_op)]
#![feature(core_intrinsics)]
#![feature(linked_list_cursors)]
#![allow(unused)]
#![feature(once_cell)]
#![feature(panic_info_message)]
extern crate core;

use crate::fs::custom::DDRIVER_PATH;
use fs::interface::*;
use std::{env, ffi::CString, mem, ptr};

pub mod buffer;
pub mod ddriver;
pub mod fs;
pub mod fuse;
pub mod utils;

#[repr(C)]
struct CustomOptions {
    device: *const libc::c_char,
}

static mut NEWFS_OPTIONS: CustomOptions = CustomOptions {
    device: ptr::null(),
};

fn get_operations() -> fuse::fuse_operations {
    let mut op = fuse::fuse_operations::empty();
    op.init = Some(rustfs_init);
    op.destroy = Some(rustfs_destory);
    op.getattr = Some(rustfs_getattr);
    op.readdir = Some(rustfs_readdir);
    op.mkdir = Some(rustfs_mkdir);
    op.mknod = Some(rustfs_mknod);
    op.write = Some(rustfs_write);
    op.read = Some(rustfs_read);
    op.access = Some(rustfs_access);
    op.unlink = Some(rustfs_unlink);
    op.rmdir = Some(rustfs_rmdir);
    op.rename = Some(rustfs_rename);
    op.utimens = Some(rustfs_utimens);
    op.truncate = Some(rustfs_truncate);
    op
}

fn main() -> ! {
    println!("rust fuse begin");
    let args_v: Vec<CString> = env::args().map(|s| CString::new(s).unwrap()).collect();
    println!("{args_v:?}");
    let mut args: Vec<*mut libc::c_char> = args_v
        .iter()
        .map(|s| s.as_ptr() as *mut libc::c_char)
        .collect();
    let mut args = fuse::fuse_args {
        argc: args.len() as i32,
        argv: args.as_mut_ptr(),
        allocated: 0,
    };
    let device_str = CString::new(DDRIVER_PATH).unwrap();
    unsafe {
        NEWFS_OPTIONS = CustomOptions {
            device: libc::strdup(device_str.as_ptr()),
        }
    };
    let templ_str = CString::new("--device=%s").unwrap();
    let option_spec: [fuse::fuse_opt; 2] = [
        fuse::fuse_opt {
            templ: templ_str.as_ptr(),
            offset: 0,
            value: 1,
        },
        fuse::fuse_opt {
            templ: ptr::null(),
            offset: 0,
            value: 0,
        },
    ];
    let mut operations = get_operations();
    unsafe {
        if fuse::fuse_opt_parse(
            &mut args,
            &raw mut NEWFS_OPTIONS as *mut libc::c_void,
            option_spec.as_ptr(),
            None,
        ) == -1
        {
            std::process::exit(-1);
        }
        println!("{:?}", args.argv);
        let ret = fuse::fuse_main_real(
            args.argc,
            args.argv as *const *const libc::c_char,
            &raw mut operations,
            mem::size_of::<fuse::fuse_operations>() as libc::size_t,
            ptr::null_mut(),
        );
        std::process::exit(ret);
    }
}
