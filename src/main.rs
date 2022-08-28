use axum::{routing::get, Router};
use error_stack::{IntoReport, ResultExt};
use hearthstone_backend::controllers::user_controller;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing::{event, Level};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "hearthstone_backend=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgres_container/postgres")
        .await
        .report()
        .attach_printable("Unable to connect to postgres database")
        .unwrap_or_else(|e| {
            event!(Level::ERROR, "{e:?}");
            std::process::exit(101)
        });

    let app = Router::with_state(pool)
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/users",
            get(user_controller::get_all_users).post(user_controller::register),
        )
        .route("/users/:id", get(user_controller::get_user_by_id))
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
