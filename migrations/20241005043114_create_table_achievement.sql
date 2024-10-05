-- Add migration script here
CREATE TABLE achievements (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description TEXT,
    image_url VARCHAR NOT NULL,
    image_id VARCHAR NOT NULL,
    game_id INTEGER REFERENCES games(id) ON DELETE SET NULL ON UPDATE CASCADE,
    community_id INTEGER REFERENCES communities(id) ON DELETE SET NULL ON UPDATE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);