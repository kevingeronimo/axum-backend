use anyhow::Context;
use async_session::MemoryStore;
use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use hearthstone_backend::{controllers::auth_controller, utils::middleware::SessionLayer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "hearthstone_backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:pass@postgres/postgres".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .context("unable to connect to database")?;

    let state = AppState {pool, key: Key::generate()};
    let mut store = MemoryStore::new();

    let app = Router::with_state(state.clone())
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", post(auth_controller::login))
        .route("/register", post(auth_controller::register))
        .layer(TraceLayer::new_for_http())
        .layer(SessionLayer::new(store, b"secret"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
    key: Key,
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(app_state: &AppState) -> Pool<Postgres> {
        app_state.pool.clone()
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(app_state: &AppState) -> Key {
        app_state.key.clone()
    }
}