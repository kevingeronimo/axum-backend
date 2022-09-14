use async_session::{Session, SessionStore};
use futures::future::BoxFuture;
use http::{header::COOKIE, request::Request, response::Response, HeaderValue};
use std::task::{Context, Poll};
use tower::{Layer, Service};

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

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let layer = self.layer.clone();

        Box::pin(async move {
            let cookie = request
                .headers()
                .get(COOKIE)
                .map(HeaderValue::to_str)
                .map(|result| {
                    result.map(|cookies| {
                        cookies
                            .split(';')
                            .filter(|cookie| cookie.contains("axum_session"))
                            .map(|cookie| cookie.rsplit_once('='))
                            .next()
                    })
                });

            let session = if let Some(Ok(Some(Some((_, cookie_value))))) = cookie {
                layer
                    .store
                    .load_session(cookie_value.to_string())
                    .await
                    .ok()
                    .flatten()
                    .and_then(Session::validate)
                    .unwrap_or_default()
            } else {
                Session::new()
            };

            let mut response = inner.call(request).await?;
            // TODO: stuff after response
            Ok(response)
        })
    }
}

#[derive(Clone)]
pub struct SessionLayer<Store> {
    store: Store,
}

impl<Store: SessionStore> SessionLayer<Store> {
    pub fn new(store: Store, _: &[u8]) -> Self {
        Self { store }
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
