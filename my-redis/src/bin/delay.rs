use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use anyhow::Result;
use crate::MainFuture::State1;

struct Delay {
    when: Instant
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("delay");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

enum MainFuture {
    State0,
    State1(Delay),
    Terminated
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use MainFuture::*;
        loop{
            match *self {
                State0 => {
                    let when = Instant::now() + Duration::from_millis(10);
                    let delay = Delay { when };
                    *self = State1(delay);
                },
                State1(ref mut delay) => {
                    return match Pin::new(delay).poll(cx) {
                        Poll::Ready(out) => {
                            assert_eq!(out, "done");
                            *self = Terminated;
                            Poll::Ready(())
                        },
                        Poll::Pending => {
                            Poll::Pending
                        }
                    }
                },
                Terminated => {
                    panic!("future ended unexpectedly");
                }
            }
        }

    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };

    let output = future.await;

    println!("output: {:?}", output);
    Ok(())
}