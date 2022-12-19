use crate::buffer::replacer::PageId;
use crate::ddriver::disk::read_page;
use crate::fs::custom::PAGE_SIZE;
use crate::fs::types::{BitMap, DEntry, Inode};
use log::debug;
use parking_lot::RwLock;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub type Data = *const RwLock<PageUnion>;

pub struct Page {
    //使用读写锁保护缓存中的页
    pub(crate) data: Box<RwLock<PageUnion>>,
    page_id: Option<PageId>,
    is_dirty: bool,
    pin_count: usize,
}

#[repr(C)]
pub union PageUnion {
    pub bytes: [u8; PAGE_SIZE],
    pub bitmap: BitMap,
    pub super_page: SuperPage,
    pub inode_page: InodePage,
    pub dir_page: DirPage,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            data: Box::new(RwLock::new(PageUnion {
                bytes: [0; PAGE_SIZE],
            })),
            page_id: None,
            is_dirty: false,
            pin_count: 0,
        }
    }
}

impl Page {
    pub fn page_id(&self) -> Option<PageId> {
        self.page_id
    }

    pub fn data(&self) -> Data {
        self.data.as_ref()
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn pin_count(&self) -> usize {
        self.pin_count
    }

    pub fn set_page_id(&mut self, page_id: PageId) {
        self.page_id = Some(page_id);
    }

    pub fn set_is_dirty(&mut self, is_dirty: bool) {
        self.is_dirty = is_dirty;
    }

    pub fn set_pin_count(&mut self, pin_count: usize) {
        self.pin_count = pin_count;
    }

    pub fn increase_pin_count(&mut self) {
        self.pin_count += 1;
    }

    pub fn decrease_pin_count(&mut self) {
        self.pin_count -= 1;
    }

    pub fn reset_data(&mut self) {
        self.data = Box::new(RwLock::new(PageUnion {
            bytes: ([0u8; PAGE_SIZE]),
        }));
    }

    pub fn init_metadata(&mut self, page_id: PageId) {
        self.set_page_id(page_id);
        self.set_is_dirty(false);
        self.set_pin_count(1);
    }
}

// 4096 / 128 = 32
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InodePage {
    pub inodes: [Inode; 32],
}

//4096 / 256 = 16
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DirPage {
    pub dir_entries: [DEntry; 16],
}

impl DirPage {
    pub fn print(&self) {
        // for i in 0..16 {
        //     debug!("------------------------------------");
        //     println!("name: {}", self.dir_entries[i].name());
        //     println!("file_type: {:?}", self.dir_entries[i].file_type);
        //     println!("inode_id: {}", self.dir_entries[i].inode_id.0);
        // }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SuperPage {
    magic_num: u32,
    sz_usage: u32,
    blank: [u8; 4088],
}

impl SuperPage {
    pub fn magic_num(&self) -> u32 {
        self.magic_num
    }

    pub fn set_magic_num(&mut self, magic_num: u32) {
        self.magic_num = magic_num;
    }

    pub fn sz_usage(&self) -> u32 {
        self.sz_usage
    }

    pub fn add_sz_usage(&mut self, sz_usage: u32) {
        self.sz_usage += sz_usage;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ddriver::disk::write_page;

    #[test]
    fn test_struct_size() {
        assert_eq!(std::mem::size_of::<PageUnion>(), PAGE_SIZE);
        assert_eq!(std::mem::size_of::<InodePage>(), PAGE_SIZE);
        assert_eq!(std::mem::size_of::<DirPage>(), PAGE_SIZE);
        assert_eq!(std::mem::size_of::<SuperPage>(), PAGE_SIZE);
    }
}
