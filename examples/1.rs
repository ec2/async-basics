/// Lets explore basic concurrency!

/// Why is this not async?
use std::thread::{sleep, spawn};
use std::time::Duration;

const EX: i32 = 2;

fn f1() {
    for i in 1..11 {
        println!("f1 {}", i);
        sleep(Duration::from_millis(100)); // Think of this as waiting for some IO
    }
}

fn f2() {
    for i in 1..11 {
        println!("f2 {}", i);
        sleep(Duration::from_millis(100)); // Think of this as waiting for some IO;
    }
}

fn main() {
    // No concurrency
    if EX == 1 {
        f1();
        f2();
    }
    // Spawns OS threads! Note that this is a preemptive concurrency.
    if EX == 2 {
        // When calling spawn, a join handler is returned.
        let joinHandler1 = spawn(f1);

        sleep(Duration::from_millis(1)); // Think of this as waiting for some IO;
        let joinHandler2 = spawn(f2);

        // Blocks current thread and waits for spawned threads to return.
        joinHandler1.join().unwrap();
        joinHandler2.join().unwrap();
    }
    println!("End!");
}
