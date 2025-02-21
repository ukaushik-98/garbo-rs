use pin_project::pin_project;
use std::marker::PhantomPinned;
use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;
use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::Sleep;
use tower::Service;

#[derive(Debug, Clone)]
struct Timeout<S> {
    inner: S,
    timeout: Duration,
}

impl<S> Timeout<S> {
    fn new(inner: S, timeout: Duration) -> Self {
        Timeout { inner, timeout }
    }
}

impl<S, Request> Service<Request> for Timeout<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let response_future = self.inner.call(request);
        let sleep = tokio::time::sleep(self.timeout);

        ResponseFuture {
            response_future,
            sleep,
        }
    }
}

#[pin_project]
struct ResponseFuture<F> {
    #[pin]
    response_future: F,
    #[pin]
    sleep: Sleep,
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.response_future.poll(cx) {
            Poll::Ready(result) => {
                let result = result.map_err(Into::into);
                return Poll::Ready(result);
            }
            Poll::Pending => {}
        }

        match this.sleep.poll(cx) {
            Poll::Ready(()) => {
                let error = Box::new(TimeoutError(()));
                return Poll::Ready(Err(error));
            }
            Poll::Pending => {}
        }

        Poll::Pending
    }
}

#[derive(Debug, Default)]
struct TimeoutError(());

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("request timed out")
    }
}

impl std::error::Error for TimeoutError {}

type BoxError = Box<dyn std::error::Error + Send + Sync>;

async fn foo() {}

async fn bar<T: Unpin>(f: T) {}

struct RandGarbo;

impl Future for RandGarbo {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

trait Service2x<Request> {
    type Response;
    type Error;
    type Future<'a>: Future<Output = Result<Self::Response, Self::Error>>
    where
        Self: 'a;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future<'_>;
}

impl<Request> Service2x<Request> for RandGarbo {
    type Response = ();
    type Error = ();
    type Future<'a>
        = Pin<Box<dyn Future<Output = Result<(), ()>> + 'a + Send>>
    where
        Self: 'a;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: Request) -> Self::Future<'_> {
        todo!()
    }
}

async fn boo<T: 'static>(x: T) {}

async fn fun() {
    let mut h = RandGarbo;
    let a = h.call(());
    boo(a);
    // tokio::spawn(async move { a });
}

struct Bax {
    inner: PhantomPinned,
}

#[tokio::main]
async fn main() {
    let test = vec!["hello"];
    let t2 = &test;
    let y = async move {
        let x = foo();
        x.await;
        println!("{:?}", &t2);
    };
    // let y = pin!(y);
    // let y = tokio::spawn(y);
    // bar(x).await;
    let bax = Bax {
        inner: PhantomPinned,
    };
    bar(bax);
    bar(RandGarbo).await;
}
