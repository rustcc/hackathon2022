pub mod disk;
mod ioctl;
mod metadata;

#[cfg(test)]
mod test {
    use crate::buffer::replacer::PageId;
    use crate::ddriver::disk::{close_ddriver, init_ddriver, read_page, write_page};
    use libc::munlock;

    #[test]
    fn test() {
        init_ddriver();
        let mut buf = [0u8; 4096];
        for i in 0..6 {
            buf[0] = i as u8;
            write_page(PageId(i), &mut buf);
        }
        for i in 0..6 {
            read_page(PageId(i), &mut buf);
            assert_eq!(buf[0], i as u8);
        }
        close_ddriver();
    }
}
