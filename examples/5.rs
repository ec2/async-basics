/// Lets explore Streams!
/// What is it?
/// A Stream is basically the async version of iterators. It is a bunch of futures.
/// single producer single consumer
use async_std::stream::Stream;
use async_std::prelude::*;
use async_std::stream;
use async_std::task::{self, sleep, Context, Poll};
use rand;
use std::pin::Pin;
use std::time::Duration;

#[derive(Debug)]
enum Food {
    Steak,
    Broccoli,
    Noodles,
}

struct Producer {
}

impl Producer {
    fn new() -> Self {
        Producer{}
    }
}

impl Stream for Producer {
    type Item = Food;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let x: u8 = rand::random::<u8>() % 4;
        if x == 0 {
            Poll::Ready(Some(Food::Steak))
        } else if x == 1 {
            Poll::Ready(Some(Food::Broccoli))
        } else if x == 2 {
            Poll::Ready(Some(Food::Noodles))
        } else {
            Poll::Ready(None)
        }
    }
}


#[async_std::main]
async fn main() {
    task::spawn(async{
        let mut p = Producer::new();
        while let Some(food) = p.next().await {
            task::sleep(Duration::from_millis(250)).await;
            println!("{:?}", food);
        }
    }).await;
    println!("End!");
}
