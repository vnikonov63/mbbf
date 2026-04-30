// Condition Variables are used for waiting for something to happen to data protected by a mutex.
// There is no possibility for the condition to get lost.
use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
    thread::{scope, sleep},
    time::Duration,
};

fn main() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        q = not_empty.wait(q).unwrap();
                    };
                };
                drop(q);
                dbg!(item);
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            sleep(Duration::from_secs(1));
        }
    });
}
