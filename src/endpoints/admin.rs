use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    Json, TypedHeader,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Serialize;

use crate::{
    models::{Media, User},
    schema::{media, users},
    utils::{authorize_and_return_user, ConnectionPool, Error},
};

pub async fn get_all_users(
    State(pool): State<ConnectionPool>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<User>>, Error> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let user = authorize_and_return_user(&mut conn, auth.0.token()).await?;
    if !user.is_admin {
        return Err(Error::Unauthorized(
            "you must be an admin to use this endpoint.".to_owned(),
        ));
    }

    Ok(Json(
        users::table
            .select(User::as_select())
            .load(&mut conn)
            .await
            .map_err(|err| Error::InternalError(err.to_string()))?,
    ))
}

#[derive(Serialize)]
// a copy of the ``Media`` struct, but without the bytes (we dont need to send it)
pub struct MediaInfo {
    pub file_name: String,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub mime_type: String,
}

pub async fn get_all_media_info(
    State(pool): State<ConnectionPool>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<MediaInfo>>, Error> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let user = authorize_and_return_user(&mut conn, auth.0.token()).await?;
    if !user.is_admin {
        return Err(Error::Unauthorized(
            "you must be an admin to use this endpoint.".to_owned(),
        ));
    }

    Ok(Json(
        media::table
            .select(Media::as_select())
            .load::<Media>(&mut conn)
            .await
            .map_err(|err| Error::InternalError(err.to_string()))?
            .iter()
            .map(|med| MediaInfo {
                file_name: med.file_name.clone(),
                user_id: med.user_id,
                created_at: med.created_at,
                mime_type: med.mime_type.clone(),
            })
            .collect(),
    ))
}
