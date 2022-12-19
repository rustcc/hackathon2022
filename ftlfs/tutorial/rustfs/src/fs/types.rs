use crate::buffer::buffer_pool_manager::AutoUnpin;
use crate::buffer::buffer_pool_manager::BPM;
use crate::buffer::page::{Data, Page};
use crate::buffer::replacer::PageId;
use crate::ddriver::disk::read_page;
use crate::fs::custom::DATA_MAP_PAGE_ID;
use crate::fs::custom::{
    DATA_START_PAGE_ID, DIR_ENTRY_PER_PAGE, INODE_START_PAGE_ID, MAX_FILE_NAME,
};
use crate::{fetch_page_read, fetch_page_write, fetch_page_write_lk};
use libc::DIR;
use log::{debug, info, trace};

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FileType {
    REG = 0,
    DIR = 1,
    SYMLINK = 2,
}

///定长256字节
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DEntry {
    pub name: [i8; MAX_FILE_NAME],
    pub file_type: FileType,
    pub inode_id: InodeId,
    pub is_valid: bool,
    blank: [u8; 119],
}

impl DEntry {
    pub fn name(&self) -> String {
        let mut name = String::new();
        for i in 0..MAX_FILE_NAME {
            if self.name[i] == 0 {
                break;
            }
            name.push(self.name[i] as u8 as char);
        }
        name
    }

    pub fn init(&mut self, name: &str, file_type: FileType, inode_id: InodeId) {
        self.is_valid = true;
        self.inode_id = inode_id;
        self.file_type = file_type;
        for (i, c) in name.chars().enumerate() {
            self.name[i] = c as i8;
        }
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::DIR
    }

    pub fn is_reg(&self) -> bool {
        self.file_type == FileType::REG
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct InodeId(pub u32);

impl InodeId {
    pub fn seek(&self) -> (usize, usize) {
        let page_id = self.0 as usize / 32 + INODE_START_PAGE_ID;
        let offset = self.0 % 32;
        (page_id, offset as usize)
    }
}

///inode为128字节
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Inode {
    pub inode_id: InodeId,
    pub file_type: FileType,
    pub direct_index: [i32; 12],
    pub indirect_index: i32,
    pub double_indirect_index: i32,
    blank: [u8; 64],
}

impl Inode {
    pub fn init_dir(&mut self, inode_id: InodeId) {
        self.inode_id = inode_id;
        self.file_type = FileType::DIR;
        self.direct_index = [-1; 12];
        self.indirect_index = -1;
        self.double_indirect_index = -1;
    }

    ///在该inode代表的目录中插入一个目录项
    pub fn add_dir_entry(&mut self, name: &str, file_type: FileType, inode_id: InodeId) {
        assert_eq!(self.file_type, FileType::DIR);
        let bpm = unsafe { BPM.as_ref().unwrap() };
        for i in 0..12 {
            let index = self.direct_index[i];
            if index == -1 {
                fetch_page_write!(
                    data_map_page: bitmap,
                    bpm,
                    DATA_MAP_PAGE_ID,
                    auto_unpin_data_map_page
                );
                self.direct_index[i] = data_map_page.alloc().unwrap() as i32;
            }
            let page_id = self.direct_index[i] as usize + DATA_START_PAGE_ID;
            fetch_page_write_lk!(dir_page: dir_page, bpm, page_id, au, lk);
            for j in 0..DIR_ENTRY_PER_PAGE {
                if !dir_page.dir_entries[j].is_valid {
                    dir_page.dir_entries[j].init(name, file_type, inode_id);
                    drop(lk);
                    return;
                }
            }
            drop(lk);
        }
    }
    ///通过inode储存的索引到目录页中搜索指定名字的目录项，返回目录项的InodeId，可以通过type_bound限制搜索的类型
    pub fn search_dir_by_name(&self, name: &str) -> Option<(InodeId, FileType)> {
        trace!("---------------search dir by name-------------------------------");
        info!("name:{}", name);
        assert_eq!(
            self.file_type,
            FileType::DIR,
            "search dir by name but not dir, inode_id:{}",
            self.inode_id.0
        );
        let bpm = unsafe { BPM.as_ref().unwrap() };
        //目录只使用直接索引，一个目录最多可支持12 * 16 = 192个目录项
        for i in 0..12 {
            let index = self.direct_index[i];
            if index == -1 {
                continue;
            }
            let page_id = self.direct_index[i] as usize + DATA_START_PAGE_ID;
            fetch_page_read!(dir_page: dir_page, bpm, page_id, au);
            info!("read dir page:{}", page_id);
            for j in 0..DIR_ENTRY_PER_PAGE {
                let dir_entry = dir_page.dir_entries[j];
                if dir_entry.is_valid && dir_entry.name() == name {
                    trace!("dir_entry.name = {}", dir_entry.name());
                    let dir_entry = dir_page.dir_entries[j];
                    return Some((dir_entry.inode_id, dir_entry.file_type));
                }
            }
        }
        None
    }

    pub fn all_dir_entry_name(&self) -> Vec<String> {
        assert_eq!(self.file_type, FileType::DIR);
        let bpm = unsafe { BPM.as_ref().unwrap() };
        let mut result = Vec::new();
        for i in 0..12 {
            let index = self.direct_index[i];
            if index == -1 {
                continue;
            }
            let page_id = self.direct_index[i] as usize + DATA_START_PAGE_ID;
            fetch_page_read!(dir_page: dir_page, bpm, page_id, au);
            for j in 0..DIR_ENTRY_PER_PAGE {
                if dir_page.dir_entries[j].is_valid {
                    debug!(
                        "dir_entry.name = {},j = {}",
                        dir_page.dir_entries[j].name(),
                        j
                    );
                    result.push(dir_page.dir_entries[j].name());
                }
            }
        }
        result
    }

    fn judge_bound(actual: FileType, bound: Option<FileType>) -> bool {
        match bound {
            Some(bound) => actual == bound,
            None => true,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::DIR
    }

    pub fn is_reg(&self) -> bool {
        self.file_type == FileType::REG
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BitMap {
    pub data: [u8; 4096],
}

impl BitMap {
    pub fn set(&mut self, bit_id: u32) {
        let byte_id = bit_id / 8;
        let bit_id = bit_id % 8;
        self.data[byte_id as usize] |= 1 << bit_id;
    }

    pub fn clear(&mut self, bit_id: u32) {
        let byte_id = bit_id / 8;
        let bit_id = bit_id % 8;
        self.data[byte_id as usize] &= !(1 << bit_id);
    }

    pub fn test(&self, bit_id: u32) -> bool {
        let byte_id = bit_id / 8;
        let bit_id = bit_id % 8;
        self.data[byte_id as usize] & (1 << bit_id) != 0
    }

    pub fn alloc(&mut self) -> Option<u32> {
        for i in 0..4096 {
            if self.data[i] != 0xff {
                for j in 0..8 {
                    if self.data[i] & (1 << j) == 0 {
                        self.data[i] |= 1 << j;
                        return Some(i as u32 * 8 + j);
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::buffer::buffer_pool_manager::{ParallelBufferPoolManager, BPM};
    use crate::fs::utils::start_flusher;

    #[test]
    fn test_struct_size() {
        assert_eq!(std::mem::size_of::<super::Inode>(), 128);
        assert_eq!(std::mem::size_of::<super::DEntry>(), 256);
    }
}
