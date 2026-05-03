// - Fundametal Theorem of Software Engineering: Every problem in computer science can
// be solved by adding a new level of indirection.
//
// - Release/Aquire Ordering should be used to make sure compiler optimizations do
// not reaorder stuff in a way that breaks our code.

use std::{
    ptr::null_mut,
    sync::atomic::{
        AtomicPtr,
        Ordering::{Acquire, Release},
    },
    thread::{current, scope, sleep},
    time::Duration,
};

use rand::prelude::*;

#[derive(Debug)]
struct Data {
    _x: i32,
    _y: String,
}

fn generate_data() -> Data {
    let mut rd = rand::rng();
    println!("Generating data on thread {:?}", current().id());
    sleep(Duration::from_millis(700));

    Data {
        _x: rd.random::<i32>(),
        _y: format!("Vasilii {}", rd.random::<i32>()),
    }
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(null_mut());

    sleep(Duration::from_millis(100));

    // Can load a not null in here.
    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        // Everuthing before Release happens-before everythign after aquire, so when we go inside the
        // if statement, we now it is after the Release stuff, namely the initialization.
        if let Err(e) = PTR.compare_exchange(null_mut(), p, Release, Acquire) {
            println!(
                "Thread {:?} participated, but lost the data race",
                current().id()
            );
            // avoiding a memory leak right here.
            drop(unsafe { Box::from_raw(p) });
            p = e;
        } else {
            println!("Won the data race {:?}", current().id());
        }
    } else {
        println!(
            "Thread {:?} did not participate in the data initialization race",
            current().id()
        );
    }

    unsafe { &*p }
}

fn main() {
    scope(|s| {
        for i in 0..10 {
            s.spawn(move || {
                if i >= 5 {
                    sleep(Duration::from_millis(700));
                }
                let data = get_data();

                println!(
                    "Thread {i} got the Data at {:p} and the data is {:?}",
                    data, data
                );
            });
        }
    })
}
