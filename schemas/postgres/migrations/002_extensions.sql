-- ResearchProcess-GPS PostgreSQL Extensions
-- Version: 002_extensions
-- Description: Optional extensions for vector search and graph operations

-- Check if pgvector is available before creating
DO $$ 
BEGIN
    IF EXISTS (SELECT 1 FROM pg_available_extensions WHERE name = 'vector') THEN
        CREATE EXTENSION IF NOT EXISTS vector;
        
        -- Add vector columns to entities
        ALTER TABLE entities ADD COLUMN IF NOT EXISTS embedding vector(1536);
        
        -- Create vector similarity index
        CREATE INDEX IF NOT EXISTS idx_entities_embedding ON entities 
            USING ivfflat (embedding vector_cosine_ops)
            WITH (lists = 100);
        
        -- Vector metadata table
        CREATE TABLE IF NOT EXISTS vector_metadata (
            entity_id UUID PRIMARY KEY,
            embedding_model VARCHAR(100),
            embedding_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            metadata JSONB,
            
            CONSTRAINT fk_vector_metadata_entity FOREIGN KEY (entity_id) 
                REFERENCES entities(id) ON DELETE CASCADE
        );
        
        -- Update features metadata
        UPDATE storage_metadata 
        SET value = value || '{"vector_search": true}'::jsonb 
        WHERE key = 'features';
        
        RAISE NOTICE 'pgvector extension enabled successfully';
    ELSE
        RAISE NOTICE 'pgvector extension not available - skipping vector search setup';
    END IF;
END $$;

-- Check if Apache AGE is available before creating
DO $$ 
BEGIN
    IF EXISTS (SELECT 1 FROM pg_available_extensions WHERE name = 'age') THEN
        CREATE EXTENSION IF NOT EXISTS age;
        
        -- Create graph schema
        CREATE SCHEMA IF NOT EXISTS graph;
        
        -- Set search path for AGE
        SET search_path = graph, ag_catalog, "$user", public;
        
        -- Create the research graph
        SELECT create_graph('research_graph');
        
        -- Create helper functions for graph operations
        CREATE OR REPLACE FUNCTION graph.create_entity_vertex(
            p_entity_id UUID,
            p_entity_type VARCHAR,
            p_properties JSONB
        ) RETURNS VOID AS $func$
        BEGIN
            -- Create vertex in AGE graph
            -- Implementation depends on AGE version
            RAISE NOTICE 'Creating vertex for entity %', p_entity_id;
        END;
        $func$ LANGUAGE plpgsql;
        
        -- Update features metadata
        UPDATE storage_metadata 
        SET value = value || '{"graph_operations": true}'::jsonb 
        WHERE key = 'features';
        
        RAISE NOTICE 'Apache AGE extension enabled successfully';
    ELSE
        RAISE NOTICE 'Apache AGE extension not available - skipping graph setup';
    END IF;
END $$;

-- Full-text search configuration
CREATE TEXT SEARCH CONFIGURATION IF NOT EXISTS genealogy_search (COPY = english);

-- Add full-text search columns
ALTER TABLE entities ADD COLUMN IF NOT EXISTS search_vector tsvector;

-- Function to update search vector
CREATE OR REPLACE FUNCTION update_search_vector()
RETURNS TRIGGER AS $$
BEGIN
    -- Extract searchable text from JSONB based on entity type
    NEW.search_vector := to_tsvector('genealogy_search',
        COALESCE(NEW.data->>'name', '') || ' ' ||
        COALESCE(NEW.data->>'question', '') || ' ' ||
        COALESCE(NEW.data->>'hypothesis', '') || ' ' ||
        COALESCE(NEW.data->>'description', '') || ' ' ||
        COALESCE(NEW.data->>'notes', '')
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for search vector
CREATE TRIGGER update_entity_search_vector
    BEFORE INSERT OR UPDATE ON entities
    FOR EACH ROW EXECUTE FUNCTION update_search_vector();

-- Index for full-text search
CREATE INDEX IF NOT EXISTS idx_entities_search ON entities USING GIN (search_vector);

-- Performance monitoring views
CREATE OR REPLACE VIEW slow_queries AS
SELECT 
    query,
    calls,
    total_exec_time,
    mean_exec_time,
    max_exec_time
FROM pg_stat_statements
WHERE mean_exec_time > 100  -- queries slower than 100ms
ORDER BY mean_exec_time DESC
LIMIT 50;

-- Table size monitoring
CREATE OR REPLACE VIEW table_sizes AS
SELECT
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) AS size,
    pg_total_relation_size(schemaname||'.'||tablename) AS size_bytes
FROM pg_tables
WHERE schemaname NOT IN ('pg_catalog', 'information_schema')
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;

-- Connection monitoring
CREATE OR REPLACE VIEW connection_stats AS
SELECT
    datname,
    usename,
    application_name,
    client_addr,
    state,
    query_start,
    state_change
FROM pg_stat_activity
WHERE datname = current_database()
ORDER BY query_start DESC;

-- Update metadata with extension status
INSERT INTO storage_metadata (key, value) 
VALUES ('extensions_setup', to_jsonb(NOW()))
ON CONFLICT (key) DO UPDATE SET value = to_jsonb(NOW());

COMMENT ON COLUMN entities.embedding IS 'Vector embedding for semantic search (requires pgvector)';
COMMENT ON COLUMN entities.search_vector IS 'Full-text search vector automatically maintained';