use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let future = MyFuture::new(42);
    println!("Final result: {}", future.await);
    Ok(())
}

#[allow(dead_code)]
fn poll_future(cx: &mut Context<'_>) -> Poll<usize> {
    let mut future = MyFuture::new(42);
    let future = Pin::new(&mut future);
    my_ready!(future.poll(cx)).into()
}

struct MyFuture {
    polled: bool,
    v: usize,
}

impl MyFuture {
    fn new(v: usize) -> Self {
        Self { polled: false, v }
    }
}

impl Future for MyFuture {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.v)
        } else {
            self.polled = true;
            // wake up the waker
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// my_ready! => Poll::Ready / Poll::Pending
#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(v) => v,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
