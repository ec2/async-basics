/// Asynchronous Functions

/// Previously, we had 2 threads that "wait for IO".
/// It was concurrent, but it was NOT async... Lets fix that

/// Introducing... Futures!
/// Futures return right away, but the value of the Future takes time to resolve.

/// The async_std library ports the Rust stdlib to use Futures.
/// Has an executor that executes Futures.
#[allow(unused_imports)]
use async_std::task::{self, sleep, spawn};
use std::time::Duration;

const EX: i32 = 1;

/// We now use the async keyword to indicate that this function is async.
/// What does it actually mean when labeling a function as async?
/// 1. You can now use the .await keyword in the function
/// 2. Modifies the function return type to Future
/// 3. Wraps the return value into a Future
/// impl std::future::Future<Output=SOMETYPE> is the real return type
async fn f1() {
    for i in 1..11 {
        println!("f1 {}", i);
        // Whats wrong with this?
        task::sleep(Duration::from_millis(1000)); // Think of this as waiting for some IO
    }
}

async fn f2() {
    for i in 1..11 {
        println!("f2 {}", i);
        task::sleep(Duration::from_millis(2000)); // Think of this as waiting for some IO;
    }
}

/// Ok, so what is this???
/// Recall that Futures need an executor to run. We have never defined an executor.
/// This attribute automagically runs main on the executor blocks on main.
/// Rewrite in part 3.
#[async_std::main]
async fn main() {
    // No concurrency
    if EX == 1
    {
        // Why does this not WORK????
        f1();
        f2();
        // Hint: Look at the warning!!!
    }
    // Spawns a Task on the Executor!
    if EX == 2
    {
        // When calling spawn, a Future is returned...
        // Spawns a "green thread"
        let join_handler1 = spawn(f1());
        // Runs f2 and blocks this green thread...
        f2().await;

        // Blocks current thread and waits for spawned threads to return.
        join_handler1.await;
    }
    println!("End!");
}

