use std::{pin::Pin, time::Duration};

struct HttpRequest {}
struct HttpResponse {}
struct Error {}

trait Handler {
    type Future: Future<Output = Result<HttpResponse, Error>>;

    fn call<'a>(&mut self, request: HttpRequest, x: &'a Vec<i32>) -> Self::Future;
}

struct RequestHandler;

impl Handler for RequestHandler {
    // We use `Pin<Box<...>>` here for simplicity, but could also define our
    // own `Future` type to avoid the overhead
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send>>;

    fn call<'a>(&mut self, _request: HttpRequest, _x: &'a Vec<i32>) -> Self::Future {
        Box::pin(async move { Ok(HttpResponse {}) })
    }
}

#[derive(Clone, Copy)]
struct Timeout<T> {
    // T will be some type that implements `Handler`
    inner_handler: T,
    duration: Duration,
}

async fn garb<'a>(x: &'a Vec<i32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse {})
}

impl<T> Handler for Timeout<T>
where
    T: Handler,
{
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send>>;

    fn call<'a>(&mut self, request: HttpRequest, x: &'a Vec<i32>) -> Self::Future {
        // Box::pin(garb(x))
        // let mut this = self;
        // Box::pin(async move {
        //     let result =
        //         tokio::time::timeout(this.duration, this.inner_handler.call(request)).await;

        //     match result {
        //         Ok(Ok(response)) => Ok(response),
        //         Ok(Err(error)) => Err(error),
        //         Err(_timeout) => todo!(),
        //     }
        // })
    }
}

#[tokio::main]
async fn main() {
    let mut timeout = Timeout {
        inner_handler: RequestHandler {},
        duration: Duration::from_millis(100),
    };
    let x = vec![];
    let _ = tokio::spawn(timeout.call(HttpRequest {}, &x)).await;
}
