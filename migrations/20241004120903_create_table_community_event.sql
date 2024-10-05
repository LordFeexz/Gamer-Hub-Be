-- Add migration script here
CREATE TYPE event_status AS ENUM ('scheduled', 'ongoing', 'completed', 'cancelled');

CREATE TABLE community_events (
    id SERIAL PRIMARY KEY,
    community_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT DEFAULT NULL,
    location VARCHAR NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP DEFAULT NULL,
    created_by UUID NOT NULL,
    is_public BOOLEAN DEFAULT FALSE,
    status event_status CHECK (status IN ('scheduled','ongoing', 'completed', 'cancelled')) NOT NULL DEFAULT 'scheduled',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_community FOREIGN KEY (community_id) REFERENCES communities (id) ON DELETE SET NULL ON UPDATE SET NULL,
    CONSTRAINT fk_user FOREIGN KEY (created_by) REFERENCES users (id) ON DELETE SET NULL ON UPDATE SET NULL
);
