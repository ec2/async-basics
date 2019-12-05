/// Lets explore Channels!
/// Multiple producers, multiple consumers
use async_std::stream::Stream;
use async_std::prelude::*;
use async_std::stream;
use async_std::sync::{Sender, Receiver, channel};
use async_std::task::{self, sleep, Context, Poll};
use rand::prelude::*;
use std::pin::Pin;
use std::time::Duration;
use std::fmt;

#[derive(Debug)]
enum Food {
    Steak,
    Broccoli,
    Noodles,
}

#[derive(Clone)]
struct Producer;

impl Stream for Producer {
    type Item = Food;

    /// Context provides a reference to the Waker
    /// Pin makes sure that this object is never moved in memory
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let x: u8 = rand::random::<u8>() % 8;
        if x == 0 {
            Poll::Ready(Some(Food::Steak))
        } else if x == 1 {
            Poll::Ready(Some(Food::Broccoli))
        } else if x == 2 {
            Poll::Ready(Some(Food::Noodles))
        } else if x >= 4 && x <= 6 {
            cx.waker().clone().wake();
            Poll::Pending
        } else {
            Poll::Ready(None)
        }
    }
}

struct Consumer;

impl Consumer {
    async fn start_consume (mut rx: Receiver<Food>) {
        println!("{:?} is starting consuming", task::current().name());
        while let Some(food) = rx.next().await {
            task::sleep(Duration::from_millis(2)).await;
            println!("{:?} is consuming: {:?}", task::current().name(), food);
        }
        println!("{:?} is finished consuming", task::current().name());
    }
}

async fn handler () {
    let (tx, rx) = channel::<Food>(10);
    let mut producers: Vec<Producer> = vec![];
    let mut consumer = Consumer;
    for i in 0u8..20 {
        producers.push(Producer);
    }
    let mut idx = 0;
    let producer_handlers = producers.into_iter().map( move |mut x| {
        let tx = tx.clone();
        idx += 1;
        task::Builder::new().name(format!("Producer {}", idx)).spawn(async move{
            println!("{:?} is starting Producing", task::current().name());
            while let Some(food) = x.next().await {
                task::sleep(Duration::from_millis(1)).await;
                println!("{:?} is Produced: {:?}", task::current().name(), food);
                // This call will have to wait if the channel is full.
                tx.send(food).await;
            }
            println!("{:?} is finished producing", task::current().name());
        }).unwrap()
    });

    let cons1 = task::Builder::new().name("consumer 1".to_string()).spawn(Consumer::start_consume(rx.clone())).unwrap();
    let cons2 = task::Builder::new().name("consumer 2".to_string()).spawn(Consumer::start_consume(rx.clone())).unwrap();

   for handler in producer_handlers{
       handler.await;
   }
    cons1.await;
    cons2.await;
}

#[async_std::main]
async fn main() {
    handler().await;
}
