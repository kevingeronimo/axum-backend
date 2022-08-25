use hearthstone_backend::controllers::user_controller;
use axum::{
    routing::get,
    Router, Extension,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@postgres_container/postgres").await?;

    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/users", get(user_controller::get_all_users))
    .layer(Extension(pool));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}