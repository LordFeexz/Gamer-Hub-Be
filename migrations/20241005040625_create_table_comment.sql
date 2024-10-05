-- Add migration script here
CREATE TABLE post_comments (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE ON UPDATE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    text TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    search_vector TSVECTOR,
    trgm_similarity FLOAT
);

CREATE INDEX "post_comments_search_vector_idx" ON post_comments USING gin("search_vector");
CREATE INDEX "post_comments_text_trgm_idx" ON post_comments USING gin (text gin_trgm_ops);

CREATE OR REPLACE FUNCTION update_post_comments_search_vector() RETURNS TRIGGER AS $$
BEGIN
NEW."search_vector" := 
    setweight(to_tsvector('english', COALESCE(NEW."text", '')), 'A') ||
    setweight(to_tsvector('indonesian', COALESCE(NEW."text", '')), 'A');

-- Update similarity score
NEW."trgm_similarity" := similarity(NEW."text", COALESCE(NEW."text", ''));

RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_post_comments_search_vector
BEFORE INSERT OR UPDATE ON post_comments
FOR EACH ROW
EXECUTE FUNCTION update_post_comments_search_vector();