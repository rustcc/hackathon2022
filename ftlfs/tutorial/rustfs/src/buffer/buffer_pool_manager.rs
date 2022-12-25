use crate::buffer::page::{Data, Page};
use crate::buffer::replacer::{FrameId, LRUReplacer, PageId, Replacer};
use crate::ddriver::disk::{close_ddriver, init_ddriver, read_page, write_page};
use crate::fs::types::InodeId;
use crate::utils::defer_guard::{set_flag, DeferGuard};
use crate::utils::semaphore::Semaphore;
use libc::free;
use log::{debug, error, info, trace, warn};
use parking_lot::Mutex;
use std::cell::LazyCell;
use std::collections::{HashMap, LinkedList};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::ptr::addr_of_mut;
use std::slice::IterMut;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct BufferPoolManager<R: Replacer<FrameId>> {
    pool_size: usize,
    num_instances: usize,
    pub instance_index: usize,
    pub inner: Mutex<BPMInner<R>>,
    pub sem: Arc<Semaphore>,
}

pub struct BPMInner<R: Replacer<FrameId>> {
    pub replacer: R,
    pub frames: Vec<Page>,
    pub page_table: HashMap<PageId, FrameId>,
    free_list: Vec<FrameId>,
}

impl<R: Replacer<FrameId>> BufferPoolManager<R> {
    ///初始化时，所有frame都在free_list中，replacer为空
    pub(crate) fn new(pool_size: usize, num_instances: usize, instance_index: usize) -> Self {
        let mut replacer = R::new(pool_size);
        let mut frames = Vec::new();
        frames.reserve(pool_size);
        for i in 0..pool_size {
            frames.push(Page::default());
        }
        let page_table = HashMap::new();
        let free_list = (0..pool_size).map(FrameId).collect();
        let inner = BPMInner {
            replacer,
            frames,
            page_table,
            free_list,
        };
        BufferPoolManager {
            pool_size,
            num_instances,
            instance_index,
            inner: Mutex::new(inner),
            sem: Arc::new(Semaphore::new(pool_size as isize)),
        }
    }

    pub fn fetch_page(&self, page_id: PageId, is_new: bool) -> Data {
        self.sem.acquire();
        let mut inner = self.inner.lock();
        if let Some(frame_id) = inner.page_table.get(&page_id).cloned() {
            let mut page = &mut inner.frames[frame_id.0];
            page.increase_pin_count();
            let result = page.data();
            if page.pin_count() == 1 {
                inner.replacer.pin(frame_id);
            } else {
                self.sem.release();
            }
            return result;
        }

        if let Some(frame_id) = inner.free_list.pop() {
            inner.page_table.insert(page_id, frame_id);
            let page: *mut Page = &mut inner.frames[frame_id.0];
            unsafe {
                (*page).init_metadata(page_id);
            }
            let mut data = unsafe { (*page).data.write() };
            drop(inner);
            if is_new {
                unsafe { data.bytes.fill(0) };
            } else {
                read_page(page_id, unsafe { &mut data.bytes });
            }
            return unsafe { (*page).data() };
        }
        let frame_id = inner.replacer.victim();
        let page: *mut Page = &mut inner.frames[frame_id.0];
        let victim_page_id = unsafe { (*page).page_id().unwrap() };
        trace!(
            "victim page id: {}, fetch page id: {}",
            victim_page_id.0,
            page_id.0
        );
        unsafe {
            (*page).init_metadata(page_id);
        }
        let result = unsafe { (*page).data() };
        inner.page_table.remove(&victim_page_id);
        inner.page_table.insert(page_id, frame_id);
        let mut data = unsafe { (*page).data.write() };
        drop(inner);
        if is_new {
            unsafe { (*data).bytes.fill(0) };
        } else {
            trace!("start read page id: {}", page_id.0);
            read_page(page_id, unsafe { &mut data.bytes });
            trace!(
                "end read page id: {},page data: {:?},page_id = {}",
                page_id.0,
                unsafe { data.bytes[0] },
                page_id.0
            );
        }
        result
    }

    pub fn unpin_page(&self, page_id: PageId, is_dirty: bool) {
        trace!("unpin page id: {}", page_id.0);
        let mut inner = self.inner.lock();
        set_flag(4);
        let mut inner = DeferGuard::new(inner, |_| set_flag(0));
        trace!("unpin page id: {},inner lock", page_id.0);
        let frame_id = *inner.page_table.get(&page_id).unwrap();
        let page = &mut inner.frames[frame_id.0];
        if !page.is_dirty() {
            page.set_is_dirty(is_dirty);
        }
        let is_dirty = page.is_dirty();

        if !is_dirty {
            page.decrease_pin_count();
            if page.pin_count() == 0 {
                inner.replacer.unpin(frame_id);
                self.sem.release();
            }
        }
    }
}

pub struct ParallelBufferPoolManager<R: Replacer<FrameId>> {
    num_instances: usize,
    pool_size: usize,
    pub(crate) instances: Vec<Box<BufferPoolManager<R>>>,
}

impl<R: Replacer<FrameId>> ParallelBufferPoolManager<R> {
    pub fn new(num_instances: usize, pool_size: usize) -> Self {
        init_ddriver();
        let mut instances = Vec::new();
        instances.reserve(num_instances);
        for i in 0..num_instances {
            instances.push(Box::new(BufferPoolManager::<R>::new(
                pool_size,
                num_instances,
                i,
            )));
        }
        Self {
            num_instances,
            pool_size,
            instances,
        }
    }

    ///采用直接映射的方式把页分散到不同的buffer pool中
    fn page_id_to_instance(&self, page_id: PageId) -> &BufferPoolManager<R> {
        if page_id.0 % 4 == 3 {
            debug!("page_id: {},instance: {}", page_id.0, 3);
        }
        self.instances[(page_id.0 % self.num_instances)].as_ref()
    }

    pub fn fetch_page(&self, page_id: PageId, is_new: bool) -> Data {
        unsafe {
            self.page_id_to_instance(page_id)
                .fetch_page(page_id, is_new)
        }
    }

    pub fn unpin_page(&self, page_id: PageId, is_dirty: bool) {
        unsafe {
            self.page_id_to_instance(page_id)
                .unpin_page(page_id, is_dirty)
        }
    }
}

impl<R: Replacer<FrameId>> Drop for ParallelBufferPoolManager<R> {
    fn drop(&mut self) {
        close_ddriver();
    }
}

pub static mut BPM: Option<ParallelBufferPoolManager<LRUReplacer<FrameId>>> = None;

pub struct AutoUnpin {
    page_id: usize,
    is_dirty: bool,
}

impl AutoUnpin {
    pub fn new(page_id: usize, is_dirty: bool) -> Self {
        AutoUnpin { page_id, is_dirty }
    }
}

impl Drop for AutoUnpin {
    fn drop(&mut self) {
        unsafe {
            info!("unpin page id: {}", self.page_id);
            BPM.as_mut()
                .unwrap()
                .unpin_page(PageId(self.page_id), self.is_dirty);
        }
    }
}

#[macro_export]
macro_rules! fetch_page_write {
    ($var:ident:$page_type:ident,$bpm:ident,$page_id: expr,$auto_unpin:ident) => {
        debug!(
            "fetch_page_write: page_id={},page_type = {},name = {}",
            $page_id,
            stringify!($page_type),
            stringify!($var)
        );
        let $var = $bpm.fetch_page(PageId($page_id), false);
        let mut $var = unsafe { (*$var).write() };
        let $var = unsafe { &mut $var.$page_type };
        let $auto_unpin = AutoUnpin::new($page_id, true);
    };
}

#[macro_export]
macro_rules! fetch_page_write_lk {
    ($var:ident:$page_type:ident,$bpm:ident,$page_id: expr,$auto_unpin:ident,$lk:ident) => {
        debug!(
            "fetch_page_write: page_id={},page_type = {},name = {}",
            $page_id,
            stringify!($page_type),
            stringify!($var)
        );
        let $lk = $bpm.fetch_page(PageId($page_id), false);
        let mut $lk = unsafe { (*$lk).write() };
        let $var = unsafe { &mut $lk.$page_type };
        let $auto_unpin = AutoUnpin::new($page_id, true);
    };
}

#[macro_export]
macro_rules! fetch_page_read {
    ($var:ident:$page_type:ident,$bpm:ident,$page_id: expr,$auto_unpin:ident) => {
        let $var = $bpm.fetch_page(PageId($page_id), false);
        let $var = unsafe { (*$var).read() };
        let $var = unsafe { &$var.$page_type };
        let $auto_unpin = AutoUnpin::new($page_id, false);
    };
}

#[macro_export]
macro_rules! new_page {
    ($var:ident:$page_type:ident,$bpm:ident,$page_id:expr,$auto_unpin:ident) => {
        let $var = $bpm.fetch_page(PageId($page_id), true);
        let mut $var = unsafe { (*$var).write() };
        let $var = unsafe { &mut $var.$page_type };
        let $auto_unpin = AutoUnpin::new($page_id, true);
    };
}
