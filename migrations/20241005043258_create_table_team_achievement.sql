-- Add migration script here
CREATE TABLE team_achievements (
    id SERIAL PRIMARY KEY NOT NULL,
    achievement_id INTEGER NOT NULL REFERENCES achievements(id) ON DELETE SET NULL ON UPDATE SET NULL,
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE SET NULL ON UPDATE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);