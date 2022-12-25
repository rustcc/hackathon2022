#![allow(dead_code)]

use crate::buffer::replacer::PageId;
use crate::ddriver::ioctl::{_IO, _IOR};
use crate::ddriver::metadata::{disk_size, fd, io_size, set_disk_size, set_fd, set_io_size};
use crate::fs::custom::PAGE_SIZE;
use crate::fs::def::PAGE_SIZE_U32;
use crate::DDRIVER_PATH;
use libc::{self, c_char, c_int, c_ulong, c_void, off_t, size_t, SEEK_SET};
use log::{debug, error, warn};
use parking_lot::Mutex;
use std::ffi::CString;

#[allow(non_camel_case_types)]
#[repr(C)]
struct ddriver_state {
    write_cnt: i32,
    read_cnt: i32,
    seek_cnt: i32,
}

const IOC_MAGIC: u32 = 'A' as u32;

const IOC_REQ_DEVICE_SIZE: u32 = _IOR::<i32>(IOC_MAGIC, 0); /* 请求查看设备大小 */
const IOC_REQ_DEVICE_STATE: u32 = _IOR::<ddriver_state>(IOC_MAGIC, 1); /* 请求设备状态，返回 ddriver_state */
const IOC_REQ_DEVICE_RESET: u32 = _IO(IOC_MAGIC, 2); /* 请求重置设备 */
const IOC_REQ_DEVICE_IO_SZ: u32 = _IOR::<i32>(IOC_MAGIC, 3); /* 请求设备IO大小 */

#[link(name = "ddriver", kind = "static")]
extern "C" {
    /**
     * 打开ddriver设备
     * path ddriver设备路径
     * int 0成功，否则失败
     */
    fn ddriver_open(path: *const c_char) -> c_int;
    /**
     *  移动ddriver磁盘头
     *  fd ddriver设备handler
     *  offset 移动到的位置，注意要和设备IO单位对齐
     *  whence SEEK_SET即可
     *  int 大于等于0成功，否则失败
     */
    fn ddriver_seek(fd: c_int, offset: off_t, whence: c_int) -> c_int;

    /**
     * 写入数据
     * fd ddriver设备handler
     * buf 要写入的数据Buf
     * size 要写入的数据大小，注意一定要等于单次设备IO单位
     * int 0成功，否则失败
     */
    fn ddriver_write(fd: c_int, buf: *const c_char, size: size_t) -> c_int;
    /**
     * 读出数据
     * fd ddriver设备handler
     * buf 要读出的数据Buf
     * size 要读出的数据大小，注意一定要等于单次设备IO单位
     * int
     */
    fn ddriver_read(fd: c_int, buf: *mut c_char, size: size_t) -> c_int;
    /**
     * ddriver IO控制
     *
     * fd ddriver设备handler
     * cmd 命令号，查看ddriver_ctl_user，IOC_开头
     * ret 返回值
     * int 0成功，否则失败
     */
    fn ddriver_ioctl(fd: c_int, cmd: c_ulong, ret: *mut c_void) -> c_int;
    /**
     * @brief 关闭ddriver设备
     *
     * @param fd ddriver设备handler
     * @return int 0成功，否则失败
     */
    fn ddriver_close(fd: c_int) -> c_int;
}

pub fn ddriver_ioctl_unwrap(flag: u32) -> u32 {
    let mut size: u32 = 0;
    let r = unsafe { ddriver_ioctl(fd(), flag as c_ulong, &raw mut size as *mut c_void) };
    assert_eq!(r, 0, "ddriver_ioctl_unwrap error");
    size
}

/// check fd != 0 and return fd
pub fn fd_unwrap() -> i32 {
    let fd = fd();
    assert_ne!(fd, 0, "ddriver not open");
    fd
}

pub fn size_check<T>() {
    debug_assert_eq!(std::mem::size_of::<T>(), PAGE_SIZE)
}

unsafe fn seek_blk(raw_blk: usize) {
    let r = ddriver_seek(fd_unwrap(), (raw_blk * io_size()) as off_t, SEEK_SET);
    assert!(r >= 0, "seek error of {raw_blk}\n");
}

unsafe fn raw_read_blk(dst: *mut c_char) {
    let r = ddriver_read(fd(), dst, io_size());
    assert!(r >= 0, "raw_read_blk err");
}

unsafe fn raw_write_blk(dst: *const c_char) {
    let r = ddriver_write(fd(), dst, io_size());
    assert!(r >= 0, "raw_write_blk err");
}

fn check_out_of_range(page_id: PageId) {
    assert!(
        page_id.0 < disk_size() / PAGE_SIZE,
        "out of ddriver range,page id = {}",
        page_id.0
    );
}

pub fn init_ddriver() {
    unsafe {
        let path = CString::new(DDRIVER_PATH).unwrap();
        let fd = ddriver_open(path.as_ptr());
        if fd < 0 {
            panic!("ddriver open failed");
        } else {
            set_fd(fd);
        }
        set_disk_size(ddriver_ioctl_unwrap(IOC_REQ_DEVICE_SIZE) as usize);
        set_io_size(ddriver_ioctl_unwrap(IOC_REQ_DEVICE_IO_SZ) as usize);
    }
}

static DRIVER_LOCK: Mutex<()> = Mutex::new(());

pub fn read_page(page_id: PageId, page: &mut [u8; PAGE_SIZE]) {
    let guard = DRIVER_LOCK.lock();
    check_out_of_range(page_id);
    let arr_ptr = (&raw mut *page) as *mut [c_char; PAGE_SIZE];
    let io_sz = io_size();
    let kio = PAGE_SIZE / io_sz;
    unsafe {
        seek_blk(page_id.0 * kio);
        for i in 0..kio {
            raw_read_blk(&raw mut *(*arr_ptr).get_unchecked_mut(i * io_sz));
        }
    }
}

pub fn write_page(page_id: PageId, page: &[u8; PAGE_SIZE]) {
    let guard = DRIVER_LOCK.lock();
    check_out_of_range(page_id);
    let arr_ptr = page as *const [u8; PAGE_SIZE] as *const [c_char; PAGE_SIZE];
    let io_sz = io_size();
    let kio = PAGE_SIZE / io_sz;
    unsafe {
        seek_blk(page_id.0 * kio);
        for i in 0..kio {
            raw_write_blk((*arr_ptr).get_unchecked(i * io_sz));
        }
    }
}

pub fn close_ddriver() {
    unsafe {
        let r = ddriver_close(fd());
        assert_eq!(r, 0, "ddriver_close error");
    }
}
