use axum::{http::StatusCode, Json};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use serde::Serialize;

pub type ConnectionPool = Pool<AsyncPgConnection>;

pub fn internal_error<E>(err: E) -> (StatusCode, Json<Response>)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(Response {
            msg: err.to_string(),
        }),
    )
}

#[derive(Serialize)]
pub struct Response {
    pub msg: String,
}