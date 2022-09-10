use std::task::{Context, Poll};
use tower::Service;

#[derive(Debug, Clone)]
pub struct SessionLayer<S> {
    inner: S,
}

impl<S> SessionLayer<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, Request> Service<Request> for SessionLayer<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        self.inner.call(request)
    }
}
