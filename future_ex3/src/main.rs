#![allow(dead_code)]
#[macro_use]
extern crate log;
use std::cell::RefCell;

thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

mod task {
    use crate::NOTIFY;

    pub struct Context<'a> {
        waker: &'a Waker,
    }

    impl<'a> Context<'a> {
        pub fn from_waker(waker: &'a Waker) -> Self {
            info!("Context from_waker");
            Context { waker }
        }

        pub fn waker(&self) -> &'a Waker {
            info!("Context waker");
            &self.waker
        }
    }

    pub struct Waker;

    impl Waker {
        pub fn wake(&self) {
            info!("Waker wake");
            NOTIFY.with(|f| *f.borrow_mut() = true)
        }
    }

}
use crate::task::*;

mod future {
    use crate::task::*;

    pub enum Poll<T> {
        Ready(T),
        Pending,
    }

    pub trait Future {
        type Output;

        fn poll(&mut self, cx: &Context) -> Poll<Self::Output>;

        fn map<U, F>(self, f: F) -> Map<Self, F>
        where
            F: FnOnce(Self::Output) -> U,
            Self: Sized,
        {
            info!("Future map");
            Map {
                future: self,
                f: Some(f),
            }
        }

        fn then<Fut, F>(self, f: F) -> Then<Self, F>
        where
            F: FnOnce(Self::Output) -> Fut,
            Fut: Future,
            Self: Sized,
        {
            info!("Future then");
            Then {
                future: self,
                f: Some(f),
            }
        }
    }

    pub trait TryFuture {
        type Ok;
        type Error;

        fn try_poll(&mut self, cx: &Context) -> Poll<Result<Self::Ok, Self::Error>>;

        fn map_err<E, F>(self, f: F) -> MapErr<Self, F>
        where
            F: FnOnce(Self::Error) -> E,
            Self: Sized,
        {
            info!("TryFuture map_err");
            MapErr {
                future: self,
                f: Some(f),
            }
        }

        fn and_then<Fut, F>(self, f: F) -> AndThen<Self, F>
        where
            F: FnOnce(Self::Ok) -> Fut,
            Fut: Future,
            Self: Sized,
        {
            info!("TryFuture and_then");
            AndThen {
                future: self,
                f: Some(f),
            }
        }
    }

    impl<F, T, E> TryFuture for F
    where
        F: Future<Output = Result<T, E>>,
    {
        type Ok = T;
        type Error = E;

        fn try_poll(&mut self, cx: &Context) -> Poll<F::Output> {
            self.poll(cx)
        }
    }

    pub struct Ready<T>(Option<T>);

    impl<T> Future for Ready<T> {
        type Output = T;

        fn poll(&mut self, _: &Context) -> Poll<Self::Output> {
            info!("Ready poll");
            Poll::Ready(self.0.take().unwrap())
        }
    }

    pub fn ready<T>(val: T) -> Ready<T> {
        info!("Ready ready");
        Ready(Some(val))
    }

    pub struct Map<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, F, T> Future for Map<Fut, F>
    where
        Fut: Future,
        F: FnOnce(Fut::Output) -> T,
    {
        type Output = T;

        fn poll(&mut self, cx: &Context) -> Poll<T> {
            info!("Map poll");
            match self.future.poll(cx) {
                Poll::Ready(val) => {
                    let f = self.f.take().unwrap();
                    Poll::Ready(f(val))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }

    pub struct Then<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, NextFut, F> Future for Then<Fut, F>
    where
        Fut: Future,
        NextFut: Future,
        F: FnOnce(Fut::Output) -> NextFut,
    {
        type Output = NextFut::Output;

        fn poll(&mut self, cx: &Context) -> Poll<Self::Output> {
            match self.future.poll(cx) {
                Poll::Ready(val) => {
                    let f = self.f.take().unwrap();
                    f(val).poll(cx)
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }

    pub struct MapErr<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, F, E> Future for MapErr<Fut, F>
    where
        Fut: TryFuture,
        F: FnOnce(Fut::Error) -> E,
    {
        type Output = Result<Fut::Ok, E>;

        fn poll(&mut self, cx: &Context) -> Poll<Self::Output> {
            match self.future.try_poll(cx) {
                Poll::Ready(result) => {
                    let f = self.f.take().unwrap();
                    Poll::Ready(result.map_err(f))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }

    pub struct AndThen<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, NextFut, F> Future for AndThen<Fut, F>
    where
        Fut: TryFuture,
        NextFut: TryFuture<Error = Fut::Error>,
        F: FnOnce(Fut::Ok) -> NextFut,
    {
        type Output = Result<NextFut::Ok, Fut::Error>;

        fn poll(&mut self, cx: &Context) -> Poll<Self::Output> {
            match self.future.try_poll(cx) {
                Poll::Ready(Ok(val)) => {
                    let f = self.f.take().unwrap();
                    f(val).try_poll(cx)
                }
                Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

use crate::future::*;

fn block_on<F>(mut f: F) -> F::Output
where
    F: Future,
{
    NOTIFY.with(|n| loop {
        if *n.borrow() {
            *n.borrow_mut() = false;
            let ctx = Context::from_waker(&Waker);
            if let Poll::Ready(val) = f.poll(&ctx) {
                return val;
            }
        }
    })
}

fn main() {
    env_logger::init();
    let my_future = future::ready(1)
        .map(|x| x + 3)
        .map(Ok)
        .map_err(|e: ()| format!("Error: {:?}", e))
        .and_then(|x| future::ready(Ok(x - 3)))
        .then(|res| {
            future::ready(match res {
                Ok(val) => Ok(val + 3),
                err => err,
            })
        });

    let val = block_on(my_future);
    assert_eq!(val, Ok(4));
}
