use axum::{
    routing::{delete, get, post},
    Router,
};
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::config::Config;
mod config;
mod endpoints;
mod models;
mod schema;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::get();

    // Kinda rigged, but this is the only way i know of to run the migrations in this circumstance (only in ).
    if cfg!(not(debug_assertions)) {
        use diesel::pg::PgConnection;
        use diesel::prelude::*;
        use diesel_migrations::MigrationHarness;

        tracing::info!("automatically running migrations.");
        const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
            diesel_migrations::embed_migrations!();

        let mut conn = PgConnection::establish(&config.database_url).unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }

    let diesel_config =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.database_url);
    let pool = Pool::builder().build(diesel_config).await.unwrap();

    let app = Router::new()
        .route("/media/upload", post(endpoints::media::upload))
        .route("/media/:name/embed", get(endpoints::media::get_image_embed))
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
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(pool);

    tracing::info!("running bokeh on bind url `http://{}`", &config.bind_addr);

    axum::Server::bind(&config.bind_addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to bind server.");
}
