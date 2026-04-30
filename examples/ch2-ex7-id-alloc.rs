use std::sync::atomic::{AtomicU32, Ordering};

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

// the internal representation of the Arc::clone follows this pattern:
fn allocate_new_id1() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    assert!(id < 1000, "too many IDs!");
    id
}

// the internal representation of the thread::scope follows this pattern
fn allocate_new_id2() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Ordering::Relaxed);
        panic!("too many IDs!")
    }
    id
}

// INTRODUCING THE MOST ADVANCED ATOMIC OPERATION compare-and-exchange.
fn allocate_new_id3() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Ordering::Relaxed);
    loop {
        assert!(id < 1000, "too many IDs!");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}

fn main() {}
