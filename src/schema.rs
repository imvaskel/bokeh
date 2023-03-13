// @generated automatically by Diesel CLI.

diesel::table! {
    media (file_name) {
        content -> Bytea,
        file_name -> Text,
        user_id -> Uuid,
        created_at -> Timestamptz,
        mime_type -> Text,
    }
}

diesel::table! {
    users (id) {
        joined_at -> Timestamptz,
        id -> Uuid,
        username -> Text,
        is_admin -> Bool,
        access_key -> Text,
    }
}

diesel::joinable!(media -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    media,
    users,
);
