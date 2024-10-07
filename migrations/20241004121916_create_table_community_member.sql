-- Add migration script here
CREATE TYPE community_role AS ENUM ('admin', 'member', 'owner');

CREATE TABLE IF NOT EXISTS community_members (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    community_id INT NOT NULL,
    role community_role DEFAULT 'member' CHECK (role IN ('admin', 'member', 'owner')),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_community FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX idx_community_members_user_id ON community_members (user_id);
CREATE INDEX idx_community_members_community_id ON community_members (community_id);