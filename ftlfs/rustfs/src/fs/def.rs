use crate::fs::custom::PAGE_SIZE;
use libc::c_int;

pub const BLOCK_SIZE: usize = 512;

pub const PAGE_SIZE_U32: u32 = PAGE_SIZE as u32;

pub const MAGIC_NUM: u32 = 0x52415455;

pub const SUCCESS: c_int = 0;
