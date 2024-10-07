-- Add migration script here
CREATE TYPE post_privacy AS ENUM (
    'public',
    'private',
    'friend-only'
);

CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    text TEXT,
    allow_comment BOOLEAN DEFAULT TRUE,
    edited_text BOOLEAN DEFAULT FALSE,
    tags TEXT[] DEFAULT '{}',
    privacy post_privacy NOT NULL DEFAULT 'public' CHECK (privacy IN ('public', 'private', 'friend-only')),
    count_like BIGINT DEFAULT 0,
    count_comment BIGINT DEFAULT 0,
    count_bookmark BIGINT DEFAULT 0,
    count_share BIGINT DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    community_id INTEGER REFERENCES communities(id) ON DELETE CASCADE ON UPDATE CASCADE,
    search_vector TSVECTOR,
    trgm_similarity FLOAT,
    is_blocked BOOLEAN DEFAULT FALSE,
    blocked_by UUID REFERENCES admins(id),
    block_reason VARCHAR(255)
);

CREATE INDEX idx_posts_search_vector ON posts USING GIN (search_vector);

CREATE OR REPLACE FUNCTION update_posts_search_vector() RETURNS TRIGGER AS $$
BEGIN
    NEW."search_vector" := 
        setweight(to_tsvector('english', COALESCE(NEW."text", '')), 'A') || 
        setweight(to_tsvector('indonesian', COALESCE(NEW."text", '')), 'A') ||
        setweight(to_tsvector('english', COALESCE(ARRAY_TO_STRING(NEW."tags", ' '), '')), 'D') ||
        setweight(to_tsvector('indonesian', COALESCE(ARRAY_TO_STRING(NEW."tags", ' '), '')), 'D');

    NEW."trgm_similarity" := greatest(
        similarity(NEW."text", COALESCE(NEW."text", '')),
        similarity(ARRAY_TO_STRING(NEW."tags", ' '), COALESCE(ARRAY_TO_STRING(NEW."tags", ' '), ''))
    );

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_posts_search_vector
BEFORE INSERT OR UPDATE ON posts
FOR EACH ROW
EXECUTE FUNCTION update_posts_search_vector();