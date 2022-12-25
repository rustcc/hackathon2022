use crate::buffer::buffer_pool_manager::AutoUnpin;
use crate::buffer::buffer_pool_manager::BPM;
use crate::buffer::replacer::PageId;
use crate::buffer::replacer::{LRUReplacer, Replacer};
use crate::fs::custom::INODE_MAP_PAGE_ID;
use crate::fs::def::SUCCESS;
use crate::fs::types::{FileType, InodeId};
use crate::{fetch_page_read, fetch_page_write, fetch_page_write_lk};
use libc::c_int;
use log::{debug, error, trace};
use std::collections::HashMap;
use std::ptr::NonNull;

pub struct DEntry {
    pub children: HashMap<String, Box<DEntry>>,
    pub father: Option<NonNull<DEntry>>,
    pub file_type: FileType,
    pub inode_id: InodeId,
    pub name: String,
}

impl DEntry {
    pub fn new(
        children: HashMap<String, Box<DEntry>>,
        father: Option<NonNull<DEntry>>,
        file_type: FileType,
        inode_id: InodeId,
        name: String,
    ) -> Self {
        Self {
            children,
            father,
            file_type,
            inode_id,
            name,
        }
    }
}

unsafe impl Send for DCache {}

unsafe impl Sync for DCache {}

pub struct DCache {
    root: DEntry,
    max_inode_num: usize,
    inode_num: usize,
    replacer: LRUReplacer<*const DEntry>,
}

impl DCache {
    pub fn new(max_inode_num: usize) -> Self {
        let root = DEntry::new(
            HashMap::new(),
            None,
            FileType::DIR,
            InodeId(0),
            String::from("/"),
        );
        let replacer = LRUReplacer::new(max_inode_num);
        Self {
            root,
            max_inode_num,
            inode_num: 0,
            replacer,
        }
    }

    /// 从根目录开始寻找指定路径的目录项
    pub fn search(&mut self, path: &str) -> Option<NonNull<DEntry>> {
        trace!("searching path: {}", path);
        let mut cur = NonNull::from(&mut self.root);
        let mut path_iter = path.split('/').peekable();
        while let Some(p) = path_iter.next() {
            if p.is_empty() {
                continue;
            }
            if let Some(child) = unsafe { cur.as_mut().children.get_mut(p) } {
                if child.file_type != FileType::DIR && path_iter.peek().is_some() {
                    return None;
                }
                cur = NonNull::from(child.as_mut());
            } else if let Some(new_node) = unsafe { self.try_load_dir_entry(cur, p) } {
                cur = new_node;
            } else {
                return None;
            }
        }
        Some(cur)
    }

    unsafe fn try_load_dir_entry(
        &mut self,
        dir: NonNull<DEntry>,
        target: &str,
    ) -> Option<NonNull<DEntry>> {
        let inode_id = dir.as_ref().inode_id;
        let bpm = BPM.as_ref().unwrap();
        let (page_id, offset) = inode_id.seek();
        fetch_page_read!(inode_page: inode_page, bpm, page_id, auto_unpin_inode_page);
        let inode = &inode_page.inodes[offset];
        let (inode_id, file_type) = inode.search_dir_by_name(target)?;
        let load_dir_entry = Box::new(DEntry::new(
            HashMap::new(),
            Some(dir),
            file_type,
            inode_id,
            target.to_string(),
        ));
        (*dir.as_ptr())
            .children
            .insert(target.to_string(), load_dir_entry);
        let ptr = (*dir.as_ptr()).children.get_mut(target).unwrap().as_mut();
        Some(NonNull::from(ptr))
    }
    /// # Safety
    /// 解引用了裸指针
    //在一个目录下插入一个目录项，先判断是否有重复
    pub unsafe fn insert(
        &mut self,
        dir: NonNull<DEntry>,
        name: &str,
        file_type: FileType,
    ) -> c_int {
        if (*dir.as_ptr()).children.get_mut(name).is_some() {
            return libc::EEXIST;
        }
        let inode_id = dir.as_ref().inode_id;
        let bpm = BPM.as_ref().unwrap();
        let (page_id, offset) = inode_id.seek();
        fetch_page_write_lk!(
            inode_page: inode_page,
            bpm,
            page_id,
            auto_unpin_inode_page,
            lk_i
        );
        let inode = &mut inode_page.inodes[offset];
        if inode.search_dir_by_name(name).is_some() {
            return libc::EEXIST;
        }
        //目录项写入磁盘
        fetch_page_write_lk!(inode_bit_map: bitmap, bpm, INODE_MAP_PAGE_ID, au, lk);
        let inode_id = inode_bit_map.alloc().unwrap();
        drop(lk);
        inode.add_dir_entry(name, file_type, InodeId(inode_id));
        //inode写入磁盘
        let (new_page_id, offset) = InodeId(inode_id).seek();
        if new_page_id != page_id {
            fetch_page_write!(inode_page: inode_page, bpm, new_page_id, au);
            let inode = &mut inode_page.inodes[offset];
            inode.init_dir(InodeId(inode_id));
        } else {
            let inode = &mut inode_page.inodes[offset];
            inode.init_dir(InodeId(inode_id));
        }
        drop(lk_i);
        //目录项写入内存
        (*dir.as_ptr()).children.insert(
            name.to_string(),
            Box::new(DEntry::new(
                HashMap::new(),
                Some(dir),
                file_type,
                InodeId(inode_id),
                name.to_string(),
            )),
        );
        debug!("insert dir entry success");
        SUCCESS
    }

    pub fn all_dir_entry_name(&self, dir: NonNull<DEntry>) -> Vec<String> {
        let inode_id = unsafe { dir.as_ref().inode_id };
        let bpm = unsafe { BPM.as_ref().unwrap() };
        let (page_id, offset) = inode_id.seek();
        fetch_page_read!(inode_page: inode_page, bpm, page_id, au);
        let inode = &inode_page.inodes[offset];
        inode.all_dir_entry_name()
    }

    pub fn print(&self) {
        Self::print_helper(&self.root);
    }

    fn print_helper(dir: &DEntry) {
        debug!("{} {}", dir.name, dir.inode_id.0);
        for (_, child) in dir.children.iter() {
            Self::print_helper(child);
        }
    }
}

pub static mut D_CACHE: Option<DCache> = None;

#[cfg(test)]
mod test {
    use crate::buffer::buffer_pool_manager::{ParallelBufferPoolManager, BPM};
    use crate::fs::dcache::{DCache, D_CACHE};
    use crate::fs::utils::start_flusher;

    #[test]
    fn test() {
        unsafe { BPM = Some(ParallelBufferPoolManager::new(4, 10)) };
        start_flusher();
        unsafe { D_CACHE = Some(DCache::new(100)) };
    }
}
