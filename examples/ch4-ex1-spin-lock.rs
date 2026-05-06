// A little rant before we dive into material.
// Consider that thread 1 locked the mutex successfully
// thread 2 tries to lock it as well, it fails. This means thread 2 is put to sleep.
// If thread 1 only holds the data for a short moment of time, there is no need
// to put the thread 2 into sleep, we can repeatedly try and lock the Mutex.
// A **spin lock** is a Mutex, where if we try to lock the mutex that is already locked, then
// spinning would start, until we recieve a successfull lock.
//
// We can give hints to the compiler on how our code should be emitted or optimized with std::hint

use std::{
    cell::UnsafeCell,
    hint::spin_loop,
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Release},
    },
};

struct SpinLock<T> {
    locked: AtomicBool,
    vlaue: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            vlaue: UnsafeCell::new(value),
        }
    }

    // this should be the GUARANTEE that it is safe to access the data
    pub fn lock(&self) {
        // In Thread that gets the lock we return the previously false value and ther is no loop
        // From the tread that tries to lock a locked SpinLock this would return true, and the loop
        // Will continue.
        while self.locked.swap(true, Acquire) {
            spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn main() {}
