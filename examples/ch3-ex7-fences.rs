// Replace Release store with 2 operations, Release fence and Relaxed store
// Replace Acquire load with 2 operations, Aquire fence and Relaxed load
// But this adds extra processor instructions.
// BUUUT fences are not only dealing with one atomic variables, they can work with many.
// SeqCst cannot be split into the fence and the operation, because stuff
// is guaranteed only for the SeqCst operation, but not the stuff before or after it
// As in the case with the release and aquire, that are more general.
//
// Disbling the compiler optimizations does not save from reordering, as there is still the
// processor which plays a role.
//
// If you remember one thing from today, let it be: "Release effectively tells the reader: "this relates to an acquire operation on the same variable"

use std::{
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Relaxed},
        fence,
    },
    thread,
    time::Duration,
};

static mut DATA: [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = 42;
            unsafe {
                DATA[i] = data;
            }
        });
        thread::sleep(Duration::from_millis(500));
        let ready: [bool; 10] = std::array::from_fn(|idx| READY[idx].load(Relaxed));
        if ready.contains(&true) {
            fence(Acquire);
            for i in 0..10 {
                if ready[i] {
                    println!("data{i}={:?}", unsafe {
                        DATA[i];
                    })
                }
            }
        }
    }
}
