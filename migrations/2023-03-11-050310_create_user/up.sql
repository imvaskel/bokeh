CREATE TABLE users (
    joined_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL UNIQUE,
    is_admin BOOLEAN DEFAULT false NOT NULL,
    access_key TEXT NOT NULL
)