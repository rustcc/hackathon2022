pub mod buffer_pool_manager;
pub mod flusher;
pub mod page;
pub mod replacer;

#[cfg(test)]
mod test {
    use crate::buffer::buffer_pool_manager::AutoUnpin;
    use crate::buffer::buffer_pool_manager::{ParallelBufferPoolManager, BPM};
    use crate::buffer::flusher::FLUSHER;
    use crate::buffer::replacer::PageId;
    use crate::fs::utils::start_flusher;
    use crate::{fetch_page_read, fetch_page_write, new_page};
    use libc::bind;
    use log::{debug, error, info, trace, warn};
    use std::io::Write;
    use std::os::unix::thread::JoinHandleExt;
    use std::time::Duration;

    #[test]
    fn test() {
        env_logger::init();
        unsafe { BPM = Some(ParallelBufferPoolManager::new(5, 2)) };
        start_flusher();
        let nthreads = 12;
        let mut handles = Vec::new();
        for i in 0..nthreads {
            let handle = std::thread::spawn(move || {
                let bpm = unsafe { BPM.as_ref().unwrap() };
                trace!("new page thread {} start", i);
                new_page!(new_page: bytes, bpm, i, auto_unpin);
                new_page[0] = i as u8;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let mut handles = Vec::new();
        for i in 0..nthreads {
            let handle = std::thread::spawn(move || {
                let bpm = unsafe { BPM.as_ref().unwrap() };
                trace!("fetch page thread {} start", i);
                fetch_page_read!(page: bytes, bpm, i, auto_unpin);
                assert_eq!(page[0], i as u8);
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let mut handles = Vec::new();
        for i in 0..nthreads {
            let handle = std::thread::spawn(move || {
                let bpm = unsafe { BPM.as_ref().unwrap() };
                fetch_page_write!(page: bytes, bpm, i, auto_unpin);
                page[0] = 6;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let mut handles = Vec::new();
        for i in 0..nthreads {
            let handle = std::thread::spawn(move || {
                let bpm = unsafe { BPM.as_ref().unwrap() };
                fetch_page_read!(page: bytes, bpm, i, auto_unpin);
                assert_eq!(page[0], 6);
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
