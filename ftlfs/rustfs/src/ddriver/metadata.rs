use crate::fs::custom::PAGE_SIZE;
use crate::fs::def::PAGE_SIZE_U32;

static mut FD: i32 = 0;

static mut IO_SIZE: usize = 0;

static mut DISK_SIZE: usize = 0;

pub unsafe fn set_fd(fd: i32) {
    FD = fd
}

pub fn fd() -> i32 {
    unsafe {
        assert_ne!(FD, 0);
        FD
    }
}

pub unsafe fn set_io_size(io_size: usize) {
    assert_ne!(io_size, 0);
    assert_eq!(PAGE_SIZE % io_size, 0);
    IO_SIZE = io_size;
}

pub fn io_size() -> usize {
    unsafe {
        assert_ne!(IO_SIZE, 0);
        IO_SIZE
    }
}

pub unsafe fn set_disk_size(disk_size: usize) {
    assert_ne!(disk_size, 0);
    DISK_SIZE = disk_size;
}

pub fn disk_size() -> usize {
    unsafe {
        assert_ne!(DISK_SIZE, 0);
        DISK_SIZE
    }
}
