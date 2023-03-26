use axum::{
    body::{Bytes, Full},
    extract::{Multipart, Path, State},
    headers::{authorization::Bearer, Authorization},
    response::{self, Html},
    Json, TypedHeader,
};
use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;
use rand::distributions::{Alphanumeric, DistString};

use crate::{
    config::Config,
    models::{CreateMedia, Media},
    schema::media,
    utils::{authorize_and_return_user, ConnectionPool, Error, Response},
};

pub async fn upload(
    State(pool): State<ConnectionPool>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut multipart: Multipart,
) -> Result<Json<Response>, Error> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let user = authorize_and_return_user(&mut conn, auth.0.token()).await?;

    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some("file") = field.name() {
            let bytes = field.bytes().await;
            if let Err(_) = bytes {
                return Err(Error::BadRequest(
                    "error getting the bytes from field `file`".to_owned(),
                ));
            }
            let content = bytes.unwrap();

            let mime = infer::get(&content).ok_or(Error::BadRequest(
                "could not determine mimetype.".to_owned(),
            ))?;
            let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
            let extension = mime.extension();
            let formatted_name = format!("{file_name}.{extension}");

            tracing::debug!(
                "user {} created new media with filename {} and with bytes len {}",
                &user.id,
                &formatted_name,
                content.len()
            );

            let data = CreateMedia {
                content: content.into(),
                file_name: &formatted_name,
                user_id: user.id,
                mime_type: mime.mime_type(),
            };

            insert_into(media::table)
                .values(&data)
                .execute(&mut conn)
                .await
                .map_err(|err| Error::InternalError(err.to_string()))?;

            return Ok(Json(Response {
                msg: formatted_name,
            }));
        }
    }

    Err(Error::BadRequest(
        "unable to find multipart field `file`.".to_owned(),
    ))
}

pub async fn get_image(
    State(pool): State<ConnectionPool>,
    Path(name): Path<String>,
) -> Result<response::Response<Full<Bytes>>, Error> {
    use crate::schema::media::dsl::*;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let matched_image: Option<Media> = media
        .filter(file_name.eq(name))
        .first(&mut conn)
        .await
        .optional()
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let image = matched_image;
    if image.is_none() {
        return Err(Error::NotFound);
    }

    let image = image.unwrap();
    tracing::debug!("media {} was viewed.", &image.file_name);
    Ok(response::Response::builder()
        .header("Content-Type", image.mime_type)
        .body(Full::from(image.content))
        .unwrap())
}

// Get the image in embed form for discord.
pub async fn get_image_embed(
    State(pool): State<ConnectionPool>,
    Path(name): Path<String>,
) -> Result<Html<String>, Error> {
    use crate::schema::media::dsl::*;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let matched_image: Option<Media> = media
        .filter(file_name.eq(name))
        .first(&mut conn)
        .await
        .optional()
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let image = matched_image;
    if image.is_none() {
        return Err(Error::NotFound);
    }

    let image = image.unwrap();
    tracing::debug!("media {} was viewed in embed.", &image.file_name);

    Ok(Html(format!(
        r#"
        <head>
            <meta property="og:image" content="{url}/media/{image}" />
            <meta property="og:url" content="{url}" />
            <meta property="og:type" content="object" />
            <meta name="description" content="Uploaded @ {time}"

            <meta name="twitter:image:src" content="{url}" />
            <meta name="twitter:description" content="Uploaded @ {time}" />
            <meta name="twitter:card" content="summary_large_image" />
        </head>
    "#,
        url = Config::get().base_url,
        image = image.file_name,
        time = image.created_at
    )))
}

pub async fn delete_image(
    State(pool): State<ConnectionPool>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(name): Path<String>,
) -> Result<Json<Response>, Error> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    let user = authorize_and_return_user(&mut conn, auth.0.token()).await?;

    let matched_image: Option<Media> = media::table
        .filter(media::file_name.eq(&name))
        .first(&mut conn)
        .await
        .optional()
        .map_err(|err| Error::InternalError(err.to_string()))?;

    if matched_image.is_none() {
        return Err(Error::NotFound);
    }

    let image = matched_image.unwrap();
    if image.user_id != user.id && !user.is_admin {
        return Err(Error::Unauthorized(
            "the uploader id does not match your id and you are not an admin.".to_owned(),
        ));
    }

    tracing::debug!(
        "image {} was deleted by user {}",
        &image.file_name,
        &user.id
    );

    diesel::delete(media::table.filter(media::file_name.eq(&name)))
        .execute(&mut conn)
        .await
        .map_err(|err| Error::InternalError(err.to_string()))?;

    Ok(Json(Response {
        msg: "media deleted.".to_owned(),
    }))
}
