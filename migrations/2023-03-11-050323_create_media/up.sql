CREATE TABLE media(
    content BLOB NOT NULL,
    file_name TEXT PRIMARY KEY NOT NULL,
    user_id UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    mime_type TEXT NOT NULL
)