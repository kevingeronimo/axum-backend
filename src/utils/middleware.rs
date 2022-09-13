use async_session::{SessionStore, MemoryStore};
use axum_extra::extract::cookie::Key;
use axum_extra::extract::SignedCookieJar;
use futures::future::BoxFuture;
use http::{request::Request, response::Response};
use std::task::{Context, Poll};
use tower::{Layer, Service};

const AXUM_SESSION_COOKIE_NAME: &str = "axum_session";

#[derive(Clone)]
pub struct SessionMiddleware<S, Store: Clone> {
    inner: S,
    layer: SessionLayer<Store>
}

impl<S, Store, ReqBody, ResBody> Service<Request<ReqBody>> for SessionMiddleware<S, Store>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    ReqBody: Send + 'static,
    S::Future: Send + 'static,
    Store: SessionStore
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        let layer = self.layer.clone();
        let jar = SignedCookieJar::from_headers(request.headers(), layer.key.clone());

        let cookie = jar
            .get(AXUM_SESSION_COOKIE_NAME)
            .map(|cookie| cookie.value().to_owned());

        let clone = self.inner.clone();

        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            if let Some(cookie_value) = cookie {
                let session = layer.store.load_session(cookie_value).await;
            } else {
                todo!()
            };

            inner.call(request).await 
        })
    }
}

#[derive(Clone)]
pub struct SessionLayer<Store> {
    store: Store,
    key: Key,
}

impl<Store> SessionLayer<Store> {
    pub fn new(store: Store, secret: &[u8]) -> Self {
        Self {
            store,
            key: Key::from(secret),
        }
    }
}

impl<S, Store: SessionStore> Layer<S> for SessionLayer<Store> {
    type Service = SessionMiddleware<S, Store>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionMiddleware {
            inner,
            layer: self.clone()
        }
    }
}
