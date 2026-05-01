use std::{
    sync::atomic::{
        AtomicI32,
        Ordering::{self, Relaxed},
    },
    thread::{scope, sleep},
    time::Duration,
};

// run with cargo run --example ch3-ex1-relaxed-ordering | grep "5 5"

fn main() {
    static X: AtomicI32 = AtomicI32::new(0);

    fn a() {
        X.fetch_add(5, Relaxed);
        sleep(Duration::from_micros(5));
        X.fetch_add(10, Relaxed);
        sleep(Duration::from_micros(5));
    }

    fn b() {
        let a = X.load(Relaxed);
        sleep(Duration::from_micros(2));
        let b = X.load(Relaxed);
        sleep(Duration::from_micros(2));
        let c = X.load(Relaxed);
        sleep(Duration::from_micros(2));
        let d = X.load(Relaxed);
        println!("{a} {b} {c} {d}");
    }

    for _ in 0..50 {
        X.store(0, Ordering::Relaxed);
        scope(|s| {
            s.spawn(a);
            s.spawn(b);
        });
    }
}
