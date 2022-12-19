use crate::buffer::buffer_pool_manager::AutoUnpin;
use crate::buffer::buffer_pool_manager::BPM;
use crate::buffer::flusher::FLUSHER;
use crate::buffer::replacer::PageId;
use crate::fetch_page_read;
use crate::fs::custom::DATA_START_PAGE_ID;
use crate::fs::types::{DEntry, FileType, InodeId};
use log::{debug, error, trace, warn};

pub fn split_path(path: &str) -> (&str, &str) {
    let mut i = path.len() - 1;
    while i > 0 && path.as_bytes()[i] != b'/' {
        i -= 1;
    }
    (&path[0..i + 1], &path[i + 1..])
}

pub fn start_flusher() {
    std::thread::spawn(|| {
        warn!("flusher tid:{}", unsafe { libc::gettid() });
        let mut flusher = unsafe { &mut FLUSHER };
        loop {
            std::thread::sleep(std::time::Duration::from_millis(10));
            flusher.copy_and_flush();
        }
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_dir() {
        assert_eq!(split_path("/a/b/c"), ("/a/b/", "c"));
        assert_eq!(split_path("/a/b"), ("/a/", "b"));
        assert_eq!(split_path("/a"), ("/", "a"));
    }
}
