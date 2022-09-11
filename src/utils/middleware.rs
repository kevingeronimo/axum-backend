use axum_extra::extract::cookie::Key;
use axum_extra::extract::SignedCookieJar;
use futures_util::ready;
use http::{request::Request, response::Response};
use pin_project::pin_project;
use std::task::{Context, Poll};
use std::{future::Future, pin::Pin};
use tower::{Layer, Service};

// type BoxError = Box<dyn std::error::Error + Send + Sync>;
const AXUM_SESSION_COOKIE_NAME: &str = "axum_session";

#[derive(Clone)]
pub struct SessionService<S> {
    inner: S,
    key: Key,
}

impl<S> SessionService<S> {
    pub fn new(inner: S, key: Key) -> Self {
        Self { inner, key }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for SessionService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        let jar = SignedCookieJar::from_headers(request.headers(), self.key.clone());

        let cookie = jar
            .get(AXUM_SESSION_COOKIE_NAME)
            .map(|cookie| cookie.value().to_owned());

        let response = self.inner.call(request);
        ResponseFuture { response }
    }
}

pub struct SessionLayer {
    key: Key,
}

impl SessionLayer {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            key: Key::from(secret),
        }
    }
}

impl<S> Layer<S> for SessionLayer {
    type Service = SessionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionService {
            inner,
            key: self.key.clone(),
        }
    }
}

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    response: F,
}

impl<F, ResBody, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let res = ready!(this.response.poll(cx)?);

        Poll::Ready(Ok(res))
    }
}
