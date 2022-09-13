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
        let layer = self.layer.clone();
        let jar = SignedCookieJar::from_headers(request.headers(), layer.key.clone());

        let cookie = jar
            .get(AXUM_SESSION_COOKIE_NAME)
            .map(|cookie| cookie.value().to_owned());

        let clone = self.inner.clone();

        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let session = layer.get_session(cookie).await;

            request.extensions_mut().insert(session);

            let mut response = inner.call(request).await?;
            let session = response.extensions_mut().get::<Session>();

            if let Some(cookie_value) = layer.save_session(session.cloned()).await {
                let c = Cookie::build(AXUM_SESSION_COOKIE_NAME, cookie_value).finish();
                let cookie_value = SignedCookieJar::new(layer.key)
                    .add(c)
                    .get(AXUM_SESSION_COOKIE_NAME)
                    .map(|cookie| cookie.to_string())
                    .unwrap();

                response
                    .headers_mut()
                    .insert(SET_COOKIE, HeaderValue::from_str(&cookie_value).unwrap());
            } else {
                tracing::error!("fail to retrieve cookie from session storage");
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            }

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
    pub fn new(store: Store, secret: &[u8]) -> Self {
        Self {
            store,
            key: Key::from(secret),
        }
    }

    async fn get_session(&self, cookie: Option<String>) -> Session {
        let session = if let Some(cookie) = cookie {
            self.store.load_session(cookie).await.ok().flatten()
        } else {
            None
        };

        session.and_then(Session::validate).unwrap_or_default()
    }

    async fn save_session(&self, session: Option<Session>) -> Option<String> {
        if let Some(session) = session {
            self.store.store_session(session).await.ok().flatten()
        } else {
            tracing::error!("please add the session as an Extension to the response");
            None
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
