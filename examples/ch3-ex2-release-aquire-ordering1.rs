// Release memory ordering applies to store operations, while Acquire memory ordering applies to load operations.
use std::{
    sync::atomic::{
        AtomicBool, AtomicU64,
        Ordering::{Acquire, Relaxed, Release},
    },
    thread::{sleep, spawn},
    time::Duration,
};

fn main() {
    static DATA: AtomicU64 = AtomicU64::new(0);
    static READY: AtomicBool = AtomicBool::new(false);

    spawn(|| {
        DATA.store(1234_1234, Relaxed);
        READY.store(true, Release);
    });

    while !READY.load(Acquire) {
        sleep(Duration::from_millis(100));
        println!("Waiting....");
    }

    println!("{}", DATA.load(Relaxed));
}
