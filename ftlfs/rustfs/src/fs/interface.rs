use crate::buffer::buffer_pool_manager::AutoUnpin;
use crate::buffer::buffer_pool_manager::{BufferPoolManager, ParallelBufferPoolManager, BPM};
use crate::buffer::page::{Page, SuperPage};
use crate::buffer::replacer::PageId;
use crate::fs::custom::{
    DATA_MAP_PAGE_ID, DATA_START_PAGE_ID, DDRIVER_PATH, INODE_MAP_PAGE_ID, PAGE_SIZE,
};
use crate::fs::dcache::{DCache, D_CACHE};
use crate::fs::def::{MAGIC_NUM, PAGE_SIZE_U32, SUCCESS};
use crate::fs::types::{DEntry, FileType, Inode, InodeId};
use crate::fs::utils::{split_path, start_flusher};
use crate::{fetch_page_write, fuse, new_page};
use libc::{
    self, blkcnt_t, blksize_t, c_char, c_int, c_uint, c_ulong, c_void, getgid, getuid, off_t,
    size_t, time, unshare, S_IFDIR, S_IFREG,
};
use log::{debug, error, info, trace, warn};
use std::alloc::alloc;
use std::ptr::null_mut;
use std::sync::{Mutex, RwLock};
use std::{ffi::CString, mem, ptr::null};

/// fuse function interface

macro_rules! cstr_convert_or_return {
    ($cstr: expr, $name: expr) => {
        match cstr_check($cstr, $name) {
            Some(s) => s,
            None => return libc::ENOENT,
        }
    };
}

fn cstr_check(cstr: *const c_char, err_output: &str) -> Option<&'static str> {
    match unsafe { std::ffi::CStr::from_ptr(cstr) }.to_str() {
        Ok(s) => Some(s),
        Err(_) => {
            println!("{err_output} utf8 err.\n");
            unsafe {
                libc::printf("try: <%s>\n\0".as_ptr().cast(), cstr);
            };
            None
        }
    }
}

pub extern "C" fn rustfs_init(_: *mut fuse::fuse_conn_info) -> c_int {
    env_logger::init();
    unsafe { BPM = Some(ParallelBufferPoolManager::new(1, 20)) };
    start_flusher();
    unsafe { D_CACHE = Some(DCache::new(100)) };
    let dir_tree = unsafe { D_CACHE.as_mut().unwrap() };
    dir_tree.print();
    unsafe {
        let bpm = BPM.as_ref().unwrap();
        fetch_page_write!(super_page: super_page, bpm, 0, auto_unpin_super_page);
        if super_page.magic_num() == MAGIC_NUM {
            trace!("find existing file system");
        } else {
            new_page!(
                inode_map_page: bitmap,
                bpm,
                INODE_MAP_PAGE_ID,
                auto_unpin_inode_map_page
            );
            inode_map_page.set(0);
            let root_dir_inode_id = InodeId(0);
            let (page_id, index) = root_dir_inode_id.seek();
            new_page!(
                root_dir_inode_page: inode_page,
                bpm,
                page_id,
                auto_unpin_root_dir_inode_page
            );
            let root_dir_inode = &mut root_dir_inode_page.inodes[index];
            root_dir_inode.init_dir(root_dir_inode_id);
            new_page!(
                data_map_page: bitmap,
                bpm,
                DATA_MAP_PAGE_ID,
                auto_unpin_data_map_page
            );
            super_page.set_magic_num(MAGIC_NUM);
        }
    }
    SUCCESS
}

pub extern "C" fn rustfs_destory(_: *mut c_void) {}
/// # Safety
/// 解引用了裸指针
pub unsafe extern "C" fn rustfs_getattr(
    path: *const c_char,
    rustfs_stat: *mut libc::stat,
) -> c_int {
    trace!("----------------------------get_attr----------------------------");
    let path = cstr_convert_or_return!(path, "rustfs_getattr");
    let stat = unsafe { &mut *rustfs_stat };
    let mut dir_tree = unsafe { D_CACHE.as_mut().unwrap() };
    let inode = unsafe { dir_tree.search(path) };
    let Some(inode) = inode else { return -libc::ENOENT; };
    if unsafe { (*inode.as_ptr()).file_type } == FileType::DIR {
        stat.st_mode = libc::S_IFDIR;
    } else if unsafe { (*inode.as_ptr()).file_type } == FileType::REG {
        stat.st_mode = libc::S_IFREG;
    }
    SUCCESS
}

pub extern "C" fn rustfs_readdir(
    path: *const c_char,
    buf: *mut c_void,
    filler: fuse::fuse_fill_dir_t,
    offset: off_t,
    _fi: *mut fuse::fuse_file_info,
) -> c_int {
    trace!("------------------------readdir------------------------");
    let path = cstr_convert_or_return!(path, "rustfs_readdir");
    let dir_tree = unsafe { D_CACHE.as_mut().unwrap() };
    let dir = unsafe { dir_tree.search(path) };
    let Some(dir) = dir else { return -libc::ENOENT; };
    let names = unsafe { dir_tree.all_dir_entry_name(dir) };
    if offset < names.len() as off_t {
        let c_name = CString::new(names[offset as usize].clone()).unwrap();
        filler(buf, c_name.as_ptr(), null(), offset + 1);
    }
    SUCCESS
}

pub extern "C" fn rustfs_mkdir(path: *const c_char, _mode: libc::mode_t) -> c_int {
    trace!("------------------------mkdir------------------------");
    unsafe {
        let path = cstr_convert_or_return!(path, "rustfs_mkdir");
        let dir_tree = unsafe { D_CACHE.as_mut().unwrap() };
        let (parent_path, name) = split_path(path);
        let dir = unsafe { dir_tree.search(parent_path) };
        let Some(dir) = dir else { return -libc::ENOENT; };
        dir_tree.insert(dir, name, FileType::DIR)
    }
}

pub extern "C" fn rustfs_mknod(
    path: *const c_char,
    _mode: libc::mode_t,
    _dev: libc::dev_t,
) -> c_int {
    trace!("------------------------mknod------------------------");
    let path = cstr_convert_or_return!(path, "rustfs_mknod");
    let dir_tree = unsafe { D_CACHE.as_mut().unwrap() };
    let (parent_path, name) = split_path(path);
    let dir = unsafe { dir_tree.search(parent_path) };
    let Some(dir) = dir else { return -libc::ENOENT; };
    unsafe { dir_tree.insert(dir, name, FileType::REG) }
}

pub extern "C" fn rustfs_write(
    path: *const c_char,
    _src: *const c_char,
    _size: size_t,
    _off: off_t,
    _info: *mut fuse::fuse_file_info,
) -> c_int {
    trace!("------------------------write------------------------");
    let _path = cstr_convert_or_return!(path, "rustfs_write");
    todo!()
}

pub extern "C" fn rustfs_read(
    path: *const c_char,
    _src: *mut c_char,
    _size: size_t,
    _off: off_t,
    _info: *mut fuse::fuse_file_info,
) -> c_int {
    trace!("------------------------read------------------------");
    let _path = cstr_convert_or_return!(path, "rustfs_read");
    todo!()
}

pub extern "C" fn rustfs_access(path: *const c_char, _typ: c_int) -> c_int {
    trace!("------------------------access------------------------");
    let path = cstr_convert_or_return!(path, "rustfs_access");
    let dir_tree = unsafe { D_CACHE.as_mut().unwrap() };
    if unsafe { dir_tree.search(path).is_some() } {
        SUCCESS
    } else {
        -libc::ENOENT
    }
}

pub extern "C" fn rustfs_unlink(path: *const c_char) -> c_int {
    trace!("------------------------unlink------------------------");
    let _path = cstr_convert_or_return!(path, "rustfs_access");
    todo!()
}

pub extern "C" fn rustfs_rmdir(path: *const c_char) -> c_int {
    trace!("------------------------rmdir------------------------");
    let _path = cstr_convert_or_return!(path, "rustfs_rmdir");
    todo!()
}

pub extern "C" fn rustfs_rename(old_name: *const c_char, new_name: *const c_char) -> c_int {
    trace!("------------------------rename------------------------");
    let _old_name = cstr_convert_or_return!(old_name, "rustfs_rename");
    let _new_name = cstr_convert_or_return!(new_name, "rustfs_rename");
    todo!()
}

pub extern "C" fn rustfs_utimens(path: *const c_char, tv: *const [libc::timespec; 2]) -> c_int {
    trace!("------------------------utimens------------------------");
    let _path = cstr_convert_or_return!(path, "rustfs_utimens");
    SUCCESS
}

pub extern "C" fn rustfs_truncate(path: *const c_char, _offset: libc::off_t) -> c_int {
    trace!("------------------------truncate------------------------");
    let _path = cstr_convert_or_return!(path, "rustfs_truncate");
    todo!()
}
