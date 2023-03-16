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
use tracing::info;
mod endpoints;
mod models;
mod schema;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();

    // Kinda rigged, but this is the only way i know of to run the migrations in this circumstance (only in ).
    if cfg!(not(debug_assertions)) {
        use diesel::pg::PgConnection;
        use diesel::prelude::*;
        use diesel_migrations::MigrationHarness;

        info!("automatically running migrations.");
        const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
            diesel_migrations::embed_migrations!();

        let mut conn = PgConnection::establish(&db_url).unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&db_url);
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
        .route("/admin/users", get(endpoints::admin::get_all_users))
        .route("/admin/media", get(endpoints::admin::get_all_media_info))
        .with_state(pool);

    let bind_addr = env::var("BIND_URL").unwrap_or("127.0.0.1:3000".to_owned());

    info!("running bokeh on bind url `http://{}`", &bind_addr);

    axum::Server::bind(&bind_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("failed to bind server.");
}
