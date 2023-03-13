use std::env;

use axum::{
    extract::{Path, State},
    headers::{authorization::Bearer, Authorization},
    Json, TypedHeader,
};
use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use uuid;

use crate::{
    models::{CreateUser, User},
    schema::media,
    utils::{ConnectionPool, Error, Response},
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
) -> Result<Json<Response>, Error> {
    use crate::schema::users::dsl::*;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let invite_key = env::var("INVITE_KEY").map_err(|err| Error::InternalError(err.to_string()))?;

    if data.key != invite_key {
        return Err(Error::Unauthorized("invite key was invalid.".to_owned()));
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
        .map_err(|err| Error::InternalError(err.to_string()))?;

    Ok(Json(Response {
        msg: user_access_key,
    }))
}

pub async fn delete_user_by_id(
    State(pool): State<ConnectionPool>,
    Path(userid): Path<String>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Response>, Error> {
    use crate::schema::users::dsl::*;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let matched_user: Option<User> = users
        .filter(access_key.eq(auth.0.token()))
        .first(&mut conn)
        .await
        .optional()
        .map_err(|err| Error::InternalError(err.to_string()))?;

    if matched_user.is_none() {
        return Err(Error::Unauthorized(
            "authorization key is invalid.".to_owned(),
        ));
    }
    let self_user = matched_user.unwrap();
    if !self_user.is_admin {
        return Err(Error::Unauthorized("you must be an admin to use this endpoint, if you are a user trying to delete your account use `/user/delete`.".to_owned()));
    }

    let uid = uuid::Uuid::parse_str(&userid).map_err(|err| Error::BadRequest(err.to_string()))?;
    let queried_user: Option<User> = users
        .filter(id.eq(&uid))
        .first(&mut conn)
        .await
        .optional()
        .map_err(|err| Error::InternalError(err.to_string()))?;

    if queried_user.is_none() {
        return Err(Error::NotFound);
    } else if queried_user.unwrap().is_admin {
        return Err(Error::Unauthorized(
            "cannot delete another admin, if you need to delete an admin, do it directly from the database."
                .to_owned(),
        ));
    }

    // first delete the users media, then delete the user
    diesel::delete(media::table.filter(media::user_id.eq(&uid)))
        .execute(&mut conn)
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    diesel::delete(users.filter(id.eq(&uid)))
        .execute(&mut conn)
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    Ok(Json(Response {
        msg: "user deleted".to_owned(),
    }))
}

pub async fn delete_user_self(
    State(pool): State<ConnectionPool>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Response>, Error> {
    use crate::schema::users::dsl::*;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let matched_user: Option<User> = users
        .filter(access_key.eq(auth.0.token()))
        .first(&mut conn)
        .await
        .optional()
        .map_err(|err| Error::InternalError(err.to_string()))?;

    if matched_user.is_none() {
        return Err(Error::Unauthorized(
            "authorization key is invalid.".to_owned(),
        ));
    }
    let user = matched_user.unwrap();

    // first delete the users media, then delete the user
    diesel::delete(media::table.filter(media::user_id.eq(&user.id)))
        .execute(&mut conn)
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    diesel::delete(users.filter(id.eq(&user.id)))
        .execute(&mut conn)
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    Ok(Json(Response {
        msg: "user deleted".to_owned(),
    }))
}
