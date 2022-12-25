use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

pub struct DeferGuard<T, G: DerefMut<Target=T>, F: FnOnce(&mut T)>(G, ManuallyDrop<F>);

impl<T, G: DerefMut<Target=T>, F: FnOnce(&mut T)> Drop for DeferGuard<T, G, F> {
    fn drop(&mut self) {
        let f = unsafe { ManuallyDrop::take(&mut self.1) };
        f(&mut *self.0);
    }
}

impl<T, G: DerefMut<Target=T>, F: FnOnce(&mut T)> DeferGuard<T, G, F> {
    pub fn new(guard: G, defer: F) -> Self {
        Self(guard, ManuallyDrop::new(defer))
    }
}

impl<T, G: DerefMut<Target=T>, F: FnOnce(&mut T)> Deref for DeferGuard<T, G, F> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T, G: DerefMut<Target=T>, F: FnOnce(&mut T)> DerefMut for DeferGuard<T, G, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

pub static mut FLAG: usize = 0;

pub fn set_flag(flag: usize) {
    unsafe {
        FLAG = flag;
    }
}

pub fn flag_inc() {
    unsafe {
        FLAG += 1;
    }
}

pub trait DerefMutDefer {
    fn defer_mut<T, F: FnOnce(&mut T)>(self, defer: F) -> DeferGuard<T, Self, F>
        where
            Self: Sized + DerefMut<Target=T>,
    {
        DeferGuard::new(self, defer)
    }
}

impl<T: DerefMut> DerefMutDefer for T {}

#[cfg(test)]
mod test {
    use crate::utils::defer_guard::DeferGuard;
    use super::*;

    #[test]
    fn test() {
        let mutex = parking_lot::Mutex::new(1);
        let mut lk = DeferGuard::new(mutex.lock(), |v| println!("v = {}", v));
        drop(lk);
        println!("end");
    }

    #[test]
    fn test_trait() {
        let mutex = parking_lot::Mutex::new(1);
        let mut lk = mutex.lock().defer_mut(|_| set_flag(0));
        set_flag(1);
    }
}
