use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread::{current, park_timeout, scope, sleep},
    time::Duration,
};

fn process_item(_: usize) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = current();

    scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Ordering::Relaxed);
                main_thread.unpark();
            }
        });

        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Working... {n}/100 done");
            // We will have an update at least every second and when the task is done
            // This may lead to the cases when the user would see the updates for each item
            // more than once.
            park_timeout(Duration::from_secs(1));
        }
    });
}
