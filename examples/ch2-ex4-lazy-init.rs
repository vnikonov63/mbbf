use std::{
    sync::atomic::{AtomicU64, Ordering::Relaxed},
    thread::sleep,
    time::Duration,
};

fn calculate_x() -> u64 {
    sleep(Duration::from_secs(6));
    1234_1234_1234
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

fn main() {
    println!("{}", get_x());
    println!("{}", get_x());
    println!("{}", get_x());
    println!("{}", get_x());
    println!("{}", get_x());
    println!("{}", get_x());
}
