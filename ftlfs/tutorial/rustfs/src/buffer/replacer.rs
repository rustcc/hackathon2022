use crate::utils::semaphore::Semaphore;
use std::collections::linked_list::CursorMut;
use std::collections::{HashMap, LinkedList};
use std::hash::Hash;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub struct FrameId(pub(crate) usize);

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash, Default)]
pub struct PageId(pub(crate) usize);

pub trait Replacer<T> {
    fn new(pool_size: usize) -> Self;
    fn victim(&mut self) -> T;

    fn pin(&mut self, resource: T);

    fn unpin(&mut self, resource: T);

    fn size(&self) -> usize;
}

#[derive(Debug)]
pub struct LRUReplacer<T: 'static + Hash + Eq + Copy> {
    container: LinkedList<T>,
    index: HashMap<T, CursorMut<'static, T>>,
}

impl<T: Hash + Eq + Copy> Replacer<T> for LRUReplacer<T> {
    fn new(pool_size: usize) -> Self {
        LRUReplacer {
            container: LinkedList::new(),
            index: HashMap::with_capacity(pool_size),
        }
    }
    //pop front
    fn victim(&mut self) -> T {
        let resource = self.container.pop_front().unwrap();
        self.index.remove(&resource);
        resource
    }

    fn pin(&mut self, resource: T) {
        // delete frame_id
        self.index
            .remove(&resource)
            .unwrap()
            .remove_current()
            .unwrap();
    }

    fn unpin(&mut self, resource: T) {
        //push back
        self.container.push_back(resource);
        self.index.insert(resource, unsafe {
            core::mem::transmute(self.container.cursor_back_mut())
        });
    }

    fn size(&self) -> usize {
        self.container.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use libc::sleep;

    #[test]
    fn replacer_test() {
        let mut replacer = LRUReplacer::new(10);
        for i in 0..10 {
            replacer.unpin(FrameId(i));
        }
        replacer.pin(FrameId(5));
        assert_eq!(replacer.victim(), FrameId(0));
        assert_eq!(replacer.victim(), FrameId(1));
        assert_eq!(replacer.victim(), FrameId(2));
        assert_eq!(replacer.victim(), FrameId(3));
        assert_eq!(replacer.victim(), FrameId(4));
        assert_eq!(replacer.victim(), FrameId(6));
        assert_eq!(replacer.victim(), FrameId(7));
        assert_eq!(replacer.victim(), FrameId(8));
        assert_eq!(replacer.victim(), FrameId(9));
    }
}
