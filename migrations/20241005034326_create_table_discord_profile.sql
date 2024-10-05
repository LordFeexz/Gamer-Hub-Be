-- Add migration script here
CREATE TABLE discord_profiles (
    id VARCHAR(255) PRIMARY KEY,
    image_url VARCHAR(255),
    banner_url VARCHAR(255),
    access_token VARCHAR(255) NOT NULL CHECK (LENGTH(access_token) > 0),
    refresh_token VARCHAR(255) NOT NULL CHECK (LENGTH(refresh_token) > 0),
    token_expires BIGINT NOT NULL,
    user_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE INDEX idx_discord_profiles_user_id ON discord_profiles (user_id);