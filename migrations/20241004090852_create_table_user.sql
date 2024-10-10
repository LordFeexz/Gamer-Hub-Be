-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    is_verified BOOLEAN DEFAULT FALSE,
    bio TEXT NULL,
    image_url VARCHAR(255) NULL,
    image_id VARCHAR(255) NULL,
    banner_image_url VARCHAR(255) NULL,
    banner_image_id VARCHAR(255) NULL,
    status VARCHAR(10) CHECK (status IN ('active', 'inActive')) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    search_vector_username TSVECTOR,
    search_vector_bio TSVECTOR,
    trgm_similarity_username FLOAT,
    trgm_similarity_bio FLOAT,
    is_blocked BOOLEAN DEFAULT FALSE,
    blocked_by UUID REFERENCES admins(id),
    block_reason VARCHAR(255)
);

CREATE INDEX "users_search_vector_username_idx" ON "users" USING gin("search_vector_username");
CREATE INDEX "users_search_vector_bio_idx" ON "users" USING gin("search_vector_bio");

CREATE INDEX "users_username_trgm_idx" ON "users" USING gin("username" gin_trgm_ops);
CREATE INDEX "users_bio_trgm_idx" ON "users" USING gin("bio" gin_trgm_ops);

CREATE OR REPLACE FUNCTION update_users_search_vectors() RETURNS TRIGGER AS $$
BEGIN
    NEW."search_vector_username" := 
    setweight(to_tsvector('english', COALESCE(NEW."username", '')), 'A') || 
    setweight(to_tsvector('indonesian', COALESCE(NEW."username", '')), 'B');
    NEW."search_vector_bio" := 
    setweight(to_tsvector('english', COALESCE(NEW."bio", '')), 'A') ||
    setweight(to_tsvector('indonesian', COALESCE(NEW."bio", '')), 'B');

    NEW."trgm_similarity_username" := similarity(COALESCE(NEW."username", ''), COALESCE(NEW."username", ''));
    NEW."trgm_similarity_bio" := similarity(COALESCE(NEW."bio", ''), COALESCE(NEW."bio", ''));

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_users_search_vectors
BEFORE INSERT OR UPDATE ON "users"
FOR EACH ROW
EXECUTE FUNCTION update_users_search_vectors();