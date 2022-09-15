use async_session::{Session, SessionStore};
use futures::future::BoxFuture;
use http::{header::{COOKIE, SET_COOKIE}, request::Request, response::Response, HeaderValue};
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

    fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
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

            // Pass the session over to the controller/handler via request extension.
            request.extensions_mut().insert(session);

            let mut response = inner.call(request).await?;

            // Read the session from the response.
            let session = response.extensions().get::<Session>();

            if let Some(session) = session {
                // The SessionStore wants ownership of the Session.
                let mut session = session.clone();
                // Cloning a Session does not clone the cookie_value so...
                session.regenerate();

                if let Some(cookie_value) = layer.store.store_session(session).await.ok().flatten()
                {
                    response.headers_mut().insert(
                        SET_COOKIE,
                        HeaderValue::from_str(&cookie_value).unwrap(),
                    );
                }
            };

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
