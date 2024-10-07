-- Add migration script here
CREATE TABLE IF NOT EXISTS games (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    code VARCHAR(255) NOT NULL UNIQUE,
    image_url VARCHAR(255) NOT NULL,
    image_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    min_player INTEGER NOT NULL DEFAULT 1 CHECK (min_player >= 1)
);

CREATE INDEX idx_games_name ON games (name);
CREATE INDEX idx_games_code ON games (code);