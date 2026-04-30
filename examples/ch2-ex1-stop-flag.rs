use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread::{sleep, spawn},
    time::Duration,
};

fn some_work() {
    println!("Doing some work!");
    sleep(Duration::from_secs(5));
}

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // Spawn a thread to do the work
    let background_thread = spawn(|| {
        while !STOP.load(Ordering::Relaxed) {
            some_work();
        }
    });

    // Use the main thread to listen to the user input
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("Commands: help, stop"),
            "stop" => break,
            cmd => println!("Unknown command: {cmd:?}"),
        }
    }

    // Inform the background thread it needs to stop
    STOP.store(true, Ordering::Relaxed);

    // Wait until the background thread finishes.
    background_thread.join().unwrap();
}
