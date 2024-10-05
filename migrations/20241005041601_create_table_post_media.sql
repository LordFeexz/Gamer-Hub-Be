-- Add migration script here
CREATE TYPE media_type AS ENUM ('image', 'video');

CREATE TABLE post_media (
    id SERIAL PRIMARY KEY,
    media_type media_type NOT NULL,
    url VARCHAR NOT NULL,
    file_id VARCHAR NOT NULL,
    post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE ON UPDATE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);