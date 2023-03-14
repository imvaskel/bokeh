use crate::schema::{media, users};
use diesel::prelude::*;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Queryable, Debug, Serialize, Selectable)]
#[diesel(table_name = media)]
pub struct Media {
    pub content: Vec<u8>,
    pub file_name: String,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub mime_type: String,
}

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub joined_at: chrono::NaiveDateTime,
    pub id: uuid::Uuid,
    pub username: String,
    pub is_admin: bool,
    pub access_key: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct CreateUser<'s> {
    pub username: &'s str,
    pub access_key: &'s str,
}

#[derive(Insertable)]
#[diesel(table_name = media)]
pub struct CreateMedia<'s> {
    pub content: Vec<u8>,
    pub file_name: &'s str,
    pub user_id: uuid::Uuid,
    pub mime_type: &'s str,
}
