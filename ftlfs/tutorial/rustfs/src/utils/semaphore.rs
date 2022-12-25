use std::fmt::Debug;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

#[derive(Debug)]
pub struct Semaphore {
    lock: Mutex<isize>,
    cvar: Condvar,
}

impl Semaphore {
    pub fn new(count: isize) -> Self {
        Semaphore {
            lock: Mutex::new(count),
            cvar: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.lock.lock().unwrap();
        while *count <= 0 {
            let result = self
                .cvar
                .wait_timeout(count, Duration::from_secs(1))
                .unwrap();
            count = result.0;
            if result.1.timed_out() {
                panic!("Semaphore acquire timeout");
            }
        }
        *count -= 1;
    }

    pub fn release(&self) {
        let mut count = self.lock.lock().unwrap();
        *count += 1;
        self.cvar.notify_one();
    }
}
