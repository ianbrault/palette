/*
 * src/semaphore.rs
 * author: ian brault <ian.brault@engineering.ucla.edu>
 */

use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    lock: Mutex<isize>,
    cond_var: Condvar,
}

impl Semaphore {
    pub fn new(count: isize) -> Semaphore {
        Semaphore {
            lock: Mutex::new(count),
            cond_var: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.lock.lock().unwrap();
        while *count <= 0 {
            count = self.cond_var.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub fn release(&self) {
        let mut count = self.lock.lock().unwrap();
        *count += 1;
        self.cond_var.notify_one();
    }
}
