use axum::{
    body::{Bytes, Full},
    extract::{Multipart, Path, State},
    headers::{authorization::Bearer, Authorization},
    http::{header, HeaderName, StatusCode},
    response, Json, TypedHeader,
};
use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;
use rand::distributions::{Alphanumeric, DistString};

use crate::{
    models::{CreateMedia, Media, User},
    schema::{media, users},
    utils::{internal_error, ConnectionPool, Response},
};

pub async fn upload(
    State(pool): State<ConnectionPool>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut multipart: Multipart,
) -> Result<Json<Response>, (StatusCode, Json<Response>)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let matched_user: Option<User> = users::table
        .filter(users::access_key.eq(auth.0.token()))
        .first(&mut conn)
        .await
        .optional()
        .map_err(internal_error)?;

    if matched_user.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(Response {
                msg: "Authorization token is invalid.".to_owned(),
            }),
        ));
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some("file") = field.name() {
            let bytes = field.bytes().await;
            if let Err(_) = bytes {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(Response {
                        msg: "error getting the bytes from field `file`".to_owned(),
                    }),
                ));
            }
            let content = bytes.unwrap();

            let mime = infer::get(&content).ok_or((
                StatusCode::BAD_REQUEST,
                Json(Response {
                    msg: "could not determine mimetype.".to_owned(),
                }),
            ))?;
            let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
            let extension = mime.extension();
            let formatted_name = format!("{file_name}.{extension}");

            let data = CreateMedia {
                content: content.into(),
                file_name: &formatted_name,
                user_id: matched_user.as_ref().unwrap().id,
                mime_type: mime.mime_type(),
            };

            insert_into(media::table)
                .values(&data)
                .execute(&mut conn)
                .await
                .map_err(internal_error)?;

            return Ok(Json(Response {
                msg: formatted_name,
            }));
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        Json(Response {
            msg: "unable to find multipart field `file`.".to_owned(),
        }),
    ))
}

pub async fn get_image(
    State(pool): State<ConnectionPool>,
    Path(name): Path<String>,
) -> Result<response::Response<Full<Bytes>>, StatusCode> {
    use crate::schema::media::dsl::*;

    let mut conn = pool.get().await;
    if let Err(_) = conn {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let matched_image: Result<Option<Media>, _> = media
        .filter(file_name.eq(name))
        .first(&mut conn.unwrap())
        .await
        .optional();

    if let Err(_) = matched_image {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let image = matched_image.unwrap();
    if image.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let image = image.unwrap();
    Ok(response::Response::builder()
        .header("Content-Type", image.mime_type)
        .body(Full::from(image.content))
        .unwrap())
}
