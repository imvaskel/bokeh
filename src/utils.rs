use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
    Json,
};
use diesel::prelude::*;
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

use crate::models::User;

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
            Error::Unauthorized(s) => (StatusCode::UNAUTHORIZED, Json(Response { msg: s })),
        }
        .into_response()
    }
}

/// Authorizes the user and returns them if they are authorized.
/// If they are not, then ``Error::Unauthorized`` will be returned.
pub async fn authorize_and_return_user(
    pool: &mut AsyncPgConnection,
    token: &str,
) -> Result<User, Error> {
    use crate::schema::users::dsl::*;

    let user: Option<User> = users
        .filter(access_key.eq(&token))
        .first(pool)
        .await
        .optional()
        .map_err(|err| Error::InternalError(err.to_string()))?;

    match user {
        Some(u) => Ok(u),
        None => Err(Error::Unauthorized(
            "authorization key is invalid.".to_owned(),
        )),
    }
}

#[derive(Serialize)]
pub struct Response {
    pub msg: String,
}
