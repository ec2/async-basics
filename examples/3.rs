use async_std::task::{self, sleep, spawn};
use std::time::Duration;

async fn f1() {
    for i in 1..11 {
        println!("f1 {}", i);
        sleep(Duration::from_millis(100)).await; // Think of this as waiting for some IO
    }
}

async fn f2() {
    for i in 1..11 {
        println!("f2 {}", i);
        sleep(Duration::from_millis(200)).await; // Think of this as waiting for some IO;
    }
}


fn main() {
    // Spawns f1 and f2 onto the executor.
    let f1_handler = spawn(f1());
    let f2_handler = spawn(f2());

    task::block_on(async {
        f1_handler.await;
        f2_handler.await;
    });

    println!("End!")
}

