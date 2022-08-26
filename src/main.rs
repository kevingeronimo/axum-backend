use axum::{routing::get, Extension, Router};
use error_stack::{IntoReport, ResultExt};
use hearthstone_backend::{
    controllers::user_controller,
    error
};
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing::{Level, event, instrument};

#[instrument]
#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "hearthstone_backend=debug,tower_http=debug",
        )
    }
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgres_container/postgres")
        .await
        .report()
        .attach_printable("Unable to connect to postgres database")
        .map_err(|e| {
            event!(Level::ERROR, "{e:?}");
            std::process::exit(101)
        });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", get(user_controller::get_all_users))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
