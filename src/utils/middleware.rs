use async_session::{Session, SessionStore};
use axum_extra::extract::cookie::{Cookie, Key};
use axum_extra::extract::SignedCookieJar;
use futures::future::BoxFuture;
use http::header::SET_COOKIE;
use http::HeaderValue;
use http::{request::Request, response::Response, StatusCode};
use std::task::{Context, Poll};
use tower::{Layer, Service};

const AXUM_SESSION_COOKIE_NAME: &str = "axum_session";

#[derive(Clone)]
pub struct SessionMiddleware<S, Store: Clone> {
    inner: S,
    layer: SessionLayer<Store>,
}

impl<S, Store, ReqBody, ResBody> Service<Request<ReqBody>> for SessionMiddleware<S, Store>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
    S::Future: Send + 'static,
    Store: SessionStore,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        // TODO: stuff before response that don't need to be awaited
        Box::pin(async move {
            // TODO: stuff before response
            let mut response = inner.call(request).await?;
            // TODO: stuff after response
            Ok(response)
        })
    }
}

#[derive(Clone)]
pub struct SessionLayer<Store> {
    store: Store,
    key: Key,
}

impl<Store: SessionStore> SessionLayer<Store> {
    pub fn new(store: Store, _: &[u8]) -> Self {
        Self {
            store,
            key: Key::generate(),
        }
    }

}

impl<S, Store: SessionStore> Layer<S> for SessionLayer<Store> {
    type Service = SessionMiddleware<S, Store>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionMiddleware {
            inner,
            layer: self.clone(),
        }
    }
}
