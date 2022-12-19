use crate::fs::def::BLOCK_SIZE;

pub const DDRIVER_PATH: &str = "/home/vpt/ddriver";
pub const PAGE_SIZE: usize = 4096;
pub const MAX_INODE_NUM: usize = PAGE_SIZE * INODE_MAP_PAGE_NUM * 8;
pub const INODE_MAP_PAGE_NUM: usize = 1;
pub const INODE_MAP_PAGE_ID: usize = 1;
pub const DATA_MAP_PAGE_NUM: usize = 1;
pub const DATA_MAP_PAGE_ID: usize = 2;
pub const INODE_START_PAGE_ID: usize = 3;
pub const DATA_START_PAGE_ID: usize = 256;
pub const INODE_SIZE: usize = 128;
pub const MAX_FILE_NAME: usize = 128;
pub const DIR_ENTRY_PER_PAGE: usize = PAGE_SIZE / 256;
