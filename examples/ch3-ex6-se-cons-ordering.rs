// Consume ordering is the weaker ordering than the release-aquire ordering, but it is unreliable.
// Sequential Memory Ordering is the strongest type of ordering
// I guarantees a globally consistent order of operations.

use std::{
    sync::atomic::{AtomicBool, Ordering::SeqCst},
    thread,
};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe {
                S.push('!');
            }
        }
    });

    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe {
                S.push('!');
            }
        }
    });

    a.join().unwrap();
    b.join().unwrap();
}
