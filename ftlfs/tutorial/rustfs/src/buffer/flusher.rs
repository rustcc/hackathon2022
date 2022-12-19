use crate::buffer::buffer_pool_manager::BPM;
use crate::buffer::page::{Page, PageUnion};
use crate::buffer::replacer::{PageId, Replacer};
use crate::ddriver::disk::write_page;
use crate::fs::custom::PAGE_SIZE;
use crate::utils::defer_guard::{set_flag, DeferGuard, FLAG, DerefMutDefer};
use crate::utils::semaphore::Semaphore;
use log::{error, info, trace, warn};
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct Flusher {
    pages: Vec<([u8; PAGE_SIZE], PageId)>,
}

pub static mut FLUSHER: Flusher = Flusher::new();

impl Flusher {
    pub const fn new() -> Self {
        Flusher { pages: Vec::new() }
    }

    pub fn copy_and_flush(&mut self) {
        let p_bpm = unsafe { BPM.as_ref().unwrap() };
        for bpm in p_bpm.instances.iter() {
            let mut dirty_pages: Vec<*mut Page> = Vec::new();
            let mut inner_lk = bpm.inner.lock();
            let mut inner = &mut *inner_lk;
            for page in inner.frames.iter_mut() {
                if page.is_dirty() {
                    dirty_pages.push(page);
                    // page.set_is_dirty(false);
                    // let page_id = page.page_id().unwrap();
                    // let data = page.data.read();
                    // self.pages.push((unsafe { data.bytes.clone() }, page_id));
                }
            }
            drop(inner_lk);
            for page in dirty_pages {
                let page_id = unsafe { (*page).page_id().unwrap() };
                let data = unsafe { (*page).data.read() };
                self.pages.push((unsafe { data.bytes }, page_id));
                unsafe { (*page).set_is_dirty(false) };
            }
            for (data, page_id) in self.pages.iter() {
                write_page(*page_id, data);
                let mut inner = &mut *bpm.inner.lock();
                set_flag(2);
                let mut inner = DeferGuard::new(inner, |_| set_flag(0));
                let frame_id = *inner.page_table.get(page_id).unwrap();
                inner.replacer.unpin(frame_id);
                inner.frames[frame_id.0].set_pin_count(0);
                bpm.sem.release();
            }
            self.pages.clear();
        }
    }
}
