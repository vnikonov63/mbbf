use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        // We know that this is safe, as we have a happens-before relationship, but the compiler
        // does not know that.
        unsafe { DATA = 1234_1234 };
        READY.store(true, Ordering::Release);
    });

    while !READY.load(Ordering::Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("Waiting....");
    }

    println!("{}", unsafe { DATA })
}
