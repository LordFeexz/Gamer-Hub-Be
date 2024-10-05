-- Add migration script here
CREATE TYPE tournament_status AS ENUM ('preparation', 'ongoing', 'completed', 'cancelled');

CREATE TABLE tournaments (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE ON UPDATE CASCADE,
    price_pool DECIMAL(15, 2) NOT NULL CHECK (price_pool > 0),
    slot INTEGER NOT NULL CHECK (slot > 0),
    start_date TIMESTAMP NOT NULL,
    registration_fee DECIMAL(15, 2) NOT NULL CHECK (registration_fee > 0),
    description TEXT DEFAULT NULL,
    status tournament_status CHECK (status IN ('preparation', 'ongoing', 'completed', 'cancelled')) NOT NULL DEFAULT 'preparation',
    image_url VARCHAR NOT NULL,
    image_id VARCHAR NOT NULL,
    location VARCHAR NOT NULL,
    tags TEXT[] NOT NULL DEFAULT '{}', 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    user_id UUID DEFAULT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    community_id INTEGER DEFAULT NULL REFERENCES communities(id) ON DELETE CASCADE ON UPDATE CASCADE,
    live_on VARCHAR DEFAULT NULL,
    is_public BOOLEAN NOT NULL DEFAULT TRUE,
    money_pool DECIMAL(15, 2) NOT NULL DEFAULT 0 CHECK (money_pool >= 0)
);