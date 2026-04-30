use std::{
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    thread::{scope, sleep},
    time::Duration,
};

fn process_item(_: i32) {
    sleep(Duration::from_millis(500));
}

fn main() {
    let num_done = AtomicUsize::new(0);

    scope(|s| {
        // A Background thread doing all of the work
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store((i + 1) as usize, Relaxed);
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working... {n}/100 done");
            sleep(Duration::from_secs(1));
        }
    });

    println!("Done");
}
