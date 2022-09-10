use http::{request::Request, response::Response};
use pin_project::pin_project;
use std::task::{Context, Poll};
use std::{future::Future, pin::Pin};
use tower::{Layer, Service};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone)]
pub struct SessionService<S> {
    inner: S,
}

impl<S> SessionService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, T, U> Service<Request<T>> for SessionService<S>
where
    S: Service<Request<T>, Response = Response<U>>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request<T>) -> Self::Future {
        let cookies = request.headers();

        let response = self.inner.call(request);
        ResponseFuture { response }
    }
}

pub struct SessionLayer;

impl<S> Layer<S> for SessionLayer {
    type Service = SessionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionService { inner }
    }
}

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    response: F,
}

impl<F, U, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<U>, E>>,
    E: Into<BoxError>,
{
    type Output = Result<Response<U>, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.response.poll(cx) {
            Poll::Ready(Ok(res)) => Poll::Ready(Ok(res)),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Pending => Poll::Pending,
        }
    }
}
