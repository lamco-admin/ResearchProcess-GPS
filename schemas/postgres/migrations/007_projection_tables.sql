-- ResearchProcess-GPS Projection Tables
-- Version: 007_projection_tables
-- Description: Create tables for event projections

-- Entity count projection
CREATE TABLE IF NOT EXISTS projection_entity_count (
    entity_type VARCHAR(100) PRIMARY KEY,
    count BIGINT NOT NULL DEFAULT 0,
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- State distribution projection
CREATE TABLE IF NOT EXISTS projection_state_distribution (
    entity_type VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    count BIGINT NOT NULL DEFAULT 0,
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (entity_type, state)
);

-- Recent activity projection
CREATE TABLE IF NOT EXISTS projection_recent_activity (
    event_id UUID PRIMARY KEY,
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    event_type VARCHAR(200) NOT NULL,
    actor_id UUID NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    summary TEXT NOT NULL
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_recent_activity_occurred 
ON projection_recent_activity(occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_recent_activity_aggregate 
ON projection_recent_activity(aggregate_id);

CREATE INDEX IF NOT EXISTS idx_recent_activity_actor 
ON projection_recent_activity(actor_id);

-- Update trigger for last_updated
CREATE OR REPLACE FUNCTION update_projection_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_updated = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Add triggers to update timestamps
CREATE TRIGGER update_entity_count_timestamp 
BEFORE UPDATE ON projection_entity_count
FOR EACH ROW EXECUTE FUNCTION update_projection_timestamp();

CREATE TRIGGER update_state_distribution_timestamp 
BEFORE UPDATE ON projection_state_distribution
FOR EACH ROW EXECUTE FUNCTION update_projection_timestamp();

-- Update metadata
UPDATE storage_metadata 
SET value = '"007"', updated_at = NOW()
WHERE key = 'schema_version';

-- Comments for documentation
COMMENT ON TABLE projection_entity_count IS 'Tracks count of entities by type';
COMMENT ON TABLE projection_state_distribution IS 'Tracks entity state distribution';
COMMENT ON TABLE projection_recent_activity IS 'Recent activity across all entities';