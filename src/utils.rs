use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
    Json,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use serde::Serialize;

pub type ConnectionPool = Pool<AsyncPgConnection>;

#[derive(Debug)]
pub enum Error {
    NotFound,
    BadRequest(String),
    InternalError(String),
    Unauthorized(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> response::Response {
        match self {
            Error::NotFound => (
                StatusCode::NOT_FOUND,
                Json(Response {
                    msg: "resource not found.".to_owned(),
                }),
            ),
            Error::BadRequest(s) => (StatusCode::BAD_REQUEST, Json(Response { msg: s })),
            Error::InternalError(s) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { msg: s }))
            }
            Error::Unauthorized(s) => (
                StatusCode::UNAUTHORIZED,
                Json(Response {
                    msg: s,
                }),
            ),
        }
        .into_response()
    }
}

#[derive(Serialize)]
pub struct Response {
    pub msg: String,
}
