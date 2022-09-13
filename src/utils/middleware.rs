use axum_extra::extract::cookie::Key;
use axum_extra::extract::SignedCookieJar;
use http::{request::Request, response::Response};
use std::{task::{Context, Poll}, pin::Pin, future::Future};
use tower::{Layer, Service};

// type BoxError = Box<dyn std::error::Error + Send + Sync>;
const AXUM_SESSION_COOKIE_NAME: &str = "axum_session";

#[derive(Clone)]
pub struct SessionMiddleware<S> {
    inner: S,
    key: Key,
}

impl<S> SessionMiddleware<S> {
    pub fn new(inner: S, key: Key) -> Self {
        Self { inner, key }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for SessionMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + 'static,
    ReqBody: 'static
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        let jar = SignedCookieJar::from_headers(request.headers(), self.key.clone());

        let cookie = jar
            .get(AXUM_SESSION_COOKIE_NAME)
            .map(|cookie| cookie.value().to_owned());

        let clone = self.inner.clone();
        
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            inner.call(request).await
        })
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
    type Service = SessionMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionMiddleware {
            inner,
            key: self.key.clone(),
        }
    }
}
