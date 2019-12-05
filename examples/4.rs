use async_std::pin::Pin;
/// Futures behind the scenes
/// How do they actually work?
/// Why don't we implement our own Futures?
use async_std::task::{self, sleep, spawn};
use futures::task::{Context, Poll};
use std::future::Future;
use std::time::Duration;

/// Lets implement some type that has the Future trait!
struct SomeFuture;

impl Future for SomeFuture {
    type Output = u8;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let x: u8 = rand::random::<u8>() % 100;
        match x {
            0..=3 => {
                println!("We aint ready yet!");
                cx.waker().clone().wake();
                Poll::Pending
            }
            _ => Poll::Ready(x),
        }
    }
}

#[async_std::main]
async fn main() {

    let f1 = SomeFuture.await;
    let f2 = SomeFuture.await;

    println!("{} {} ", f1, f2);

    println!("End!")
}
