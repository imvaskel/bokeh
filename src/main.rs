#![allow(dead_code)]

use std::env;

use axum::{
    extract::{Multipart, State},
    routing::{get, post},
    Router,
};
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection
};
use dotenvy::dotenv;
use utils::ConnectionPool;
mod endpoints;
mod models;
mod schema;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(env::var("DATABASE_URL").unwrap());
    let pool = Pool::builder().build(config).await.unwrap();

    let app = Router::new()
        .route("/media/upload", post(endpoints::media::upload))
        .route("/media/:name", get(endpoints::media::get_image))
        .route("/user/register", post(endpoints::user::register_user))
        .with_state(pool);

    axum::Server::bind(
        &env::var("BIND_URL")
            .unwrap_or("127.0.0.1:3000".to_owned())
            .parse()
            .unwrap(),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}