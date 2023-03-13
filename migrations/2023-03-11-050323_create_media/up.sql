CREATE TABLE media (
    content bytea NOT NULL,
    file_name TEXT PRIMARY KEY NOT NULL,
    user_id UUID REFERENCES users(id) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    mime_type TEXT NOT NULL
)