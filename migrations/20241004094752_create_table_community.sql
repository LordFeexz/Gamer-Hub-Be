-- Add migration script here
CREATE TABLE IF NOT EXISTS communities (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT DEFAULT NULL,
    image_url VARCHAR DEFAULT NULL,
    image_id VARCHAR DEFAULT NULL,
    owner UUID NOT NULL,
    is_discord_server BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    search_vector_name TSVECTOR,
    search_vector_description TSVECTOR,
    name_trgm_similarity FLOAT,
    description_trgm_similarity FLOAT,
    FOREIGN KEY (owner) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX communities_search_vector_name_idx ON Communities USING gin(search_vector_name);
CREATE INDEX communities_search_vector_description_idx ON Communities USING gin(search_vector_description);

CREATE OR REPLACE FUNCTION update_communities_search_vectors() RETURNS TRIGGER AS $$
BEGIN
    NEW.search_vector_name := 
        setweight(to_tsvector('english', COALESCE(NEW.name, '')), 'A') || 
        setweight(to_tsvector('indonesian', COALESCE(NEW.name, '')), 'B');
    NEW.search_vector_description := 
        setweight(to_tsvector('english', COALESCE(NEW.description, '')), 'A') ||
        setweight(to_tsvector('indonesian', COALESCE(NEW.description, '')), 'B');
    NEW.name_trgm_similarity := similarity(NEW.name, COALESCE(NEW.name, ''));
    NEW.description_trgm_similarity := similarity(NEW.description, COALESCE(NEW.description, ''));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER trigger_update_communities_search_vectors
BEFORE INSERT OR UPDATE ON Communities
FOR EACH ROW
EXECUTE FUNCTION update_communities_search_vectors();
