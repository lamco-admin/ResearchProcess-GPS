-- ResearchProcess-GPS PostgreSQL Partitioning Strategy
-- Version: 003_partitioning
-- Description: Partitioning setup for large datasets

-- Convert changes table to partitioned table (by month)
-- This needs to be done carefully in production with data migration

-- Create partitioned changes table
CREATE TABLE IF NOT EXISTS changes_partitioned (
    id UUID NOT NULL,
    entity_id UUID NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    workspace_id UUID,
    operation VARCHAR(20) NOT NULL CHECK (operation IN ('CREATE', 'UPDATE', 'DELETE', 'STATE_TRANSITION')),
    changed_by UUID NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    before_data JSONB,
    after_data JSONB,
    metadata JSONB,
    PRIMARY KEY (id, changed_at)
) PARTITION BY RANGE (changed_at);

-- Create indexes on partitioned table
CREATE INDEX idx_changes_part_entity ON changes_partitioned(entity_id);
CREATE INDEX idx_changes_part_changed_at ON changes_partitioned(changed_at);
CREATE INDEX idx_changes_part_operation ON changes_partitioned(operation);

-- Function to create monthly partitions
CREATE OR REPLACE FUNCTION create_monthly_partition(table_name text, start_date date)
RETURNS void AS $$
DECLARE
    partition_name text;
    start_timestamp timestamp;
    end_timestamp timestamp;
BEGIN
    partition_name := table_name || '_' || to_char(start_date, 'YYYY_MM');
    start_timestamp := start_date;
    end_timestamp := start_date + interval '1 month';
    
    EXECUTE format('
        CREATE TABLE IF NOT EXISTS %I PARTITION OF %I
        FOR VALUES FROM (%L) TO (%L)',
        partition_name, table_name, start_timestamp, end_timestamp
    );
    
    RAISE NOTICE 'Created partition % for % to %', partition_name, start_timestamp, end_timestamp;
END;
$$ LANGUAGE plpgsql;

-- Create partitions for the next 12 months
DO $$
DECLARE
    current_month date;
    i integer;
BEGIN
    current_month := date_trunc('month', CURRENT_DATE);
    
    FOR i IN 0..11 LOOP
        PERFORM create_monthly_partition('changes_partitioned', current_month + (i || ' months')::interval);
    END LOOP;
END $$;

-- Automated partition management
CREATE OR REPLACE FUNCTION maintain_partitions()
RETURNS void AS $$
DECLARE
    next_month date;
    partition_name text;
    oldest_partition text;
    retention_months integer := 24; -- Keep 2 years of data
BEGIN
    -- Create next month's partition
    next_month := date_trunc('month', CURRENT_DATE + interval '1 month');
    PERFORM create_monthly_partition('changes_partitioned', next_month);
    
    -- Drop old partitions beyond retention period
    FOR oldest_partition IN
        SELECT tablename
        FROM pg_tables
        WHERE schemaname = 'public'
        AND tablename LIKE 'changes_partitioned_%'
        AND tablename < 'changes_partitioned_' || to_char(CURRENT_DATE - (retention_months || ' months')::interval, 'YYYY_MM')
        ORDER BY tablename
    LOOP
        EXECUTE format('DROP TABLE IF EXISTS %I', oldest_partition);
        RAISE NOTICE 'Dropped old partition %', oldest_partition;
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- Schedule partition maintenance (requires pg_cron extension)
-- Run this if pg_cron is available:
-- SELECT cron.schedule('maintain-partitions', '0 0 1 * *', 'SELECT maintain_partitions()');

-- Entity archival for inactive data
CREATE TABLE IF NOT EXISTS entities_archive (
    LIKE entities INCLUDING ALL
);

-- Function to archive old entities
CREATE OR REPLACE FUNCTION archive_old_entities(older_than interval DEFAULT '2 years')
RETURNS integer AS $$
DECLARE
    archived_count integer;
BEGIN
    -- Move entities that haven't been updated in specified time to archive
    WITH archived AS (
        DELETE FROM entities
        WHERE updated_at < CURRENT_TIMESTAMP - older_than
        AND deleted_at IS NOT NULL
        RETURNING *
    )
    INSERT INTO entities_archive
    SELECT * FROM archived;
    
    GET DIAGNOSTICS archived_count = ROW_COUNT;
    
    RAISE NOTICE 'Archived % entities older than %', archived_count, older_than;
    RETURN archived_count;
END;
$$ LANGUAGE plpgsql;

-- Sharding support preparation (for future horizontal scaling)
-- This creates a shard key that can be used for distributing data
ALTER TABLE entities ADD COLUMN IF NOT EXISTS shard_key INTEGER GENERATED ALWAYS AS (
    abs(hashtext(id::text)) % 16
) STORED;

CREATE INDEX idx_entities_shard ON entities(shard_key);

-- Table for tracking large binary data separately
CREATE TABLE IF NOT EXISTS binary_storage (
    id UUID PRIMARY KEY,
    entity_id UUID NOT NULL,
    chunk_index INTEGER NOT NULL,
    data BYTEA NOT NULL,
    total_chunks INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT fk_binary_storage_entity FOREIGN KEY (entity_id)
        REFERENCES entities(id) ON DELETE CASCADE,
    CONSTRAINT uk_entity_chunk UNIQUE (entity_id, chunk_index)
);

-- Function to store large binary data in chunks
CREATE OR REPLACE FUNCTION store_large_binary(
    p_entity_id UUID,
    p_data BYTEA,
    p_chunk_size INTEGER DEFAULT 1048576  -- 1MB chunks
) RETURNS INTEGER AS $$
DECLARE
    total_size INTEGER;
    num_chunks INTEGER;
    chunk_data BYTEA;
    i INTEGER;
BEGIN
    total_size := octet_length(p_data);
    num_chunks := ceil(total_size::numeric / p_chunk_size);
    
    -- Delete existing chunks if any
    DELETE FROM binary_storage WHERE entity_id = p_entity_id;
    
    -- Store chunks
    FOR i IN 0..(num_chunks - 1) LOOP
        chunk_data := substring(p_data FROM (i * p_chunk_size + 1) FOR p_chunk_size);
        
        INSERT INTO binary_storage (id, entity_id, chunk_index, data, total_chunks)
        VALUES (gen_random_uuid(), p_entity_id, i, chunk_data, num_chunks);
    END LOOP;
    
    -- Store reference in main entity table
    UPDATE entities 
    SET binary_data = NULL  -- Clear inline storage
    WHERE id = p_entity_id;
    
    RETURN num_chunks;
END;
$$ LANGUAGE plpgsql;

-- Function to retrieve large binary data
CREATE OR REPLACE FUNCTION retrieve_large_binary(p_entity_id UUID)
RETURNS BYTEA AS $$
DECLARE
    result BYTEA := '';
    chunk RECORD;
BEGIN
    FOR chunk IN
        SELECT data
        FROM binary_storage
        WHERE entity_id = p_entity_id
        ORDER BY chunk_index
    LOOP
        result := result || chunk.data;
    END LOOP;
    
    RETURN result;
END;
$$ LANGUAGE plpgsql;

-- Update storage metadata
UPDATE storage_metadata 
SET value = value || '{"partitioning": true, "archival": true}'::jsonb 
WHERE key = 'features';

COMMENT ON TABLE changes_partitioned IS 'Partitioned change tracking table for better performance at scale';
COMMENT ON TABLE entities_archive IS 'Archive table for old/deleted entities';
COMMENT ON TABLE binary_storage IS 'Chunked storage for large binary data';