#[macro_use]
extern crate log;

use std::cell::RefCell;
use std::{thread::sleep, time::Duration};

thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

struct Context<'a> {
    waker: &'a Waker,
}

impl<'a> Context<'a> {
    fn from_waker(waker: &'a Waker) -> Self {
        info!("Context from_waker");
        Context { waker }
    }

    fn waker(&self) -> &'a Waker {
        info!("Context waker");
        &self.waker
    }
}

struct Waker;

impl Waker {
    fn wake(&self) {
        NOTIFY.with(|f| {
            info!("Waker wake:{} ",*f.borrow());
             *f.borrow_mut() = true;
         })
    }
}

enum Poll<T> {
    Ready(T),
    Pending,
}

trait Future {
    type Output;

    fn poll(&mut self, cx: &Context) -> Poll<Self::Output>;
}

#[derive(Default)]
struct MyFuture {
    count: u32,
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        info!("impl Future for MyFuture: {}",self.count);
        sleep(Duration::from_secs(1));
        match self.count {
            3 => Poll::Ready(3),
            _ => {
                self.count += 1;
                ctx.waker().wake();
                Poll::Pending
            }
        }
    }
}

fn run<F>(mut f: F) -> F::Output
where
    F: Future,
{
    NOTIFY.with(|n| loop {
        info!("run...");
        if *n.borrow() {
            *n.borrow_mut() = false;
            let ctx = Context::from_waker(&Waker);
            info!("before Poll...");
            if let Poll::Ready(val) = f.poll(&ctx) {
                return val;
            }
        }
    })
}

fn main() {
    env_logger::init();
    let my_future = MyFuture::default();
    println!("Output: {}", run(my_future));
}
