use std::env;

use axum::{extract::State, http::StatusCode, Json};
use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

use crate::{
    models::{CreateUser, User},
    utils::{internal_error, ConnectionPool, Response},
};

/// This file holds the endpoints for /user/:x.

#[derive(Deserialize)]
pub struct UserRegisterData {
    username: String,
    key: String,
}

pub async fn register_user(
    State(pool): State<ConnectionPool>,
    Json(data): Json<UserRegisterData>,
) -> Result<Json<Response>, (StatusCode, Json<Response>)> {
    use crate::schema::users::dsl::*;

    let mut conn = pool.get().await.map_err(internal_error)?;

    let matched_user: Option<User> = users
        .filter(username.eq(&data.username))
        .first(&mut conn)
        .await
        .optional()
        .map_err(internal_error)?;

    if let Some(_) = matched_user {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(Response {
                msg: "That username was already taken".to_owned(),
            }),
        ));
    }

    let invite_key = env::var("INVITE_KEY").map_err(internal_error)?;

    if data.key != invite_key {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(Response {
                msg: "Invalid invite key.".to_owned(),
            }),
        ));
    }

    let user_access_key = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
    let data = CreateUser {
        username: &data.username,
        access_key: &user_access_key,
    };

    insert_into(users)
        .values(&data)
        .execute(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(Response {
        msg: user_access_key,
    }))
}
