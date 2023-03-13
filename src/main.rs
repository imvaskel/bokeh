#![allow(dead_code)]

use std::env;

use axum::{
    routing::{delete, get, post},
    Router,
};
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use dotenvy::dotenv;
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
        .route(
            "/media/delete/:name",
            delete(endpoints::media::delete_image),
        )
        .route("/user/register", post(endpoints::user::register_user))
        .route(
            "/user/delete/:user",
            delete(endpoints::user::delete_user_by_id),
        )
        .route("/user/delete/", delete(endpoints::user::delete_user_self))
        .with_state(pool);

    let bind_addr = env::var("BIND_URL").unwrap_or("127.0.0.1:3000".to_owned());

    println!("running server on {bind_addr}.");

    axum::Server::bind(&bind_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("failed to bind server.");
}
