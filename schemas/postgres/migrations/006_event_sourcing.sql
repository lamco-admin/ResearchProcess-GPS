-- ResearchProcess-GPS Event Sourcing Schema
-- Version: 006_event_sourcing
-- Description: Enhanced event store for event sourcing pattern

-- Event store table for all domain events
CREATE TABLE events (
    -- Event identification
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL UNIQUE, -- Public event ID (for idempotency)
    
    -- Aggregate information
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    aggregate_version BIGINT NOT NULL,
    
    -- Event information
    event_type VARCHAR(200) NOT NULL,
    event_version INT NOT NULL DEFAULT 1, -- Event schema version
    
    -- Event data
    event_data JSONB NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    
    -- Causation and correlation
    correlation_id UUID,
    causation_id UUID,
    
    -- Actor and timing
    actor_id UUID NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Workspace context
    workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE,
    
    -- Indexing hints
    tags TEXT[] DEFAULT '{}',
    
    -- Constraints
    CONSTRAINT uk_aggregate_version UNIQUE (aggregate_id, aggregate_version),
    CONSTRAINT chk_event_version CHECK (event_version > 0),
    CONSTRAINT chk_aggregate_version CHECK (aggregate_version > 0)
);

-- Event snapshots for performance
CREATE TABLE event_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    aggregate_version BIGINT NOT NULL,
    
    -- Snapshot data
    snapshot_data JSONB NOT NULL,
    snapshot_metadata JSONB NOT NULL DEFAULT '{}',
    
    -- Timing
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Make sure we only have one snapshot per aggregate/version
    CONSTRAINT uk_snapshot_aggregate_version UNIQUE (aggregate_id, aggregate_version)
);

-- Projections tracking table
CREATE TABLE projections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_name VARCHAR(200) NOT NULL UNIQUE,
    projection_type VARCHAR(100) NOT NULL,
    
    -- Current state
    last_processed_event_id UUID,
    last_processed_at TIMESTAMPTZ,
    current_position BIGINT DEFAULT 0,
    
    -- Status tracking
    status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
    error_count INT DEFAULT 0,
    last_error TEXT,
    last_error_at TIMESTAMPTZ,
    
    -- Configuration
    config JSONB NOT NULL DEFAULT '{}',
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT chk_status CHECK (status IN ('ACTIVE', 'PAUSED', 'ERROR', 'REBUILDING'))
);

-- Projection checkpoints for recovery
CREATE TABLE projection_checkpoints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID NOT NULL REFERENCES projections(id) ON DELETE CASCADE,
    event_id UUID NOT NULL,
    position BIGINT NOT NULL,
    
    -- Checkpoint data
    state JSONB NOT NULL DEFAULT '{}',
    
    -- Timing
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Keep limited checkpoints per projection
    CONSTRAINT uk_projection_position UNIQUE (projection_id, position)
);

-- Sagas/Process managers state
CREATE TABLE sagas (
    id UUID PRIMARY KEY,
    saga_type VARCHAR(100) NOT NULL,
    
    -- State management
    current_state VARCHAR(100) NOT NULL,
    state_data JSONB NOT NULL DEFAULT '{}',
    
    -- Correlation
    correlation_id UUID NOT NULL,
    
    -- Timing
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
    
    -- Workspace context
    workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE,
    
    CONSTRAINT chk_saga_status CHECK (status IN ('ACTIVE', 'COMPLETED', 'FAILED', 'TIMEOUT'))
);

-- Commands for CQRS pattern
CREATE TABLE commands (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    command_id UUID NOT NULL UNIQUE, -- Public command ID
    
    -- Command information
    command_type VARCHAR(200) NOT NULL,
    aggregate_id UUID,
    
    -- Command data
    command_data JSONB NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    
    -- Actor and timing
    actor_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Execution tracking
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    executed_at TIMESTAMPTZ,
    result JSONB,
    error TEXT,
    
    -- Workspace context
    workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE,
    
    CONSTRAINT chk_command_status CHECK (status IN ('PENDING', 'PROCESSING', 'COMPLETED', 'FAILED', 'REJECTED'))
);

-- Create indexes for performance
-- Event indexes
CREATE INDEX idx_events_aggregate ON events(aggregate_id, aggregate_version);
CREATE INDEX idx_events_type ON events(event_type);
CREATE INDEX idx_events_occurred ON events(occurred_at);
CREATE INDEX idx_events_correlation ON events(correlation_id) WHERE correlation_id IS NOT NULL;
CREATE INDEX idx_events_causation ON events(causation_id) WHERE causation_id IS NOT NULL;
CREATE INDEX idx_events_actor ON events(actor_id);
CREATE INDEX idx_events_workspace ON events(workspace_id) WHERE workspace_id IS NOT NULL;
CREATE INDEX idx_events_tags ON events USING GIN(tags) WHERE array_length(tags, 1) > 0;

-- Snapshot indexes
CREATE INDEX idx_snapshots_aggregate ON event_snapshots(aggregate_id);
CREATE INDEX idx_snapshots_created ON event_snapshots(created_at);

-- Projection indexes
CREATE INDEX idx_projections_status ON projections(status);
CREATE INDEX idx_projection_checkpoints_projection ON projection_checkpoints(projection_id);

-- Saga indexes
CREATE INDEX idx_sagas_type_status ON sagas(saga_type, status);
CREATE INDEX idx_sagas_correlation ON sagas(correlation_id);

-- Command indexes
CREATE INDEX idx_commands_type_status ON commands(command_type, status);
CREATE INDEX idx_commands_aggregate ON commands(aggregate_id) WHERE aggregate_id IS NOT NULL;
CREATE INDEX idx_commands_created ON commands(created_at);

-- Function to get current aggregate version
CREATE OR REPLACE FUNCTION get_aggregate_version(p_aggregate_id UUID)
RETURNS BIGINT AS $$
DECLARE
    v_version BIGINT;
BEGIN
    SELECT COALESCE(MAX(aggregate_version), 0)
    INTO v_version
    FROM events
    WHERE aggregate_id = p_aggregate_id;
    
    RETURN v_version;
END;
$$ LANGUAGE plpgsql;

-- Function to append event with version check
CREATE OR REPLACE FUNCTION append_event(
    p_event_id UUID,
    p_aggregate_id UUID,
    p_aggregate_type VARCHAR,
    p_expected_version BIGINT,
    p_event_type VARCHAR,
    p_event_data JSONB,
    p_metadata JSONB,
    p_actor_id UUID,
    p_occurred_at TIMESTAMPTZ,
    p_correlation_id UUID DEFAULT NULL,
    p_causation_id UUID DEFAULT NULL,
    p_workspace_id UUID DEFAULT NULL,
    p_tags TEXT[] DEFAULT '{}'
)
RETURNS BIGINT AS $$
DECLARE
    v_current_version BIGINT;
    v_new_version BIGINT;
BEGIN
    -- Get current version with lock
    -- First lock the aggregate by selecting any row for this aggregate
    PERFORM 1 FROM events 
    WHERE aggregate_id = p_aggregate_id 
    FOR UPDATE;
    
    -- Then get the max version
    SELECT COALESCE(MAX(aggregate_version), 0)
    INTO v_current_version
    FROM events
    WHERE aggregate_id = p_aggregate_id;
    
    -- Check expected version (-1 means no check)
    IF p_expected_version != -1 AND v_current_version != p_expected_version THEN
        RAISE EXCEPTION 'Concurrency conflict: expected version % but current is %', 
            p_expected_version, v_current_version;
    END IF;
    
    -- Calculate new version
    v_new_version := v_current_version + 1;
    
    -- Insert event
    INSERT INTO events (
        event_id, aggregate_id, aggregate_type, aggregate_version,
        event_type, event_data, metadata, actor_id, occurred_at,
        correlation_id, causation_id, workspace_id, tags
    ) VALUES (
        p_event_id, p_aggregate_id, p_aggregate_type, v_new_version,
        p_event_type, p_event_data, p_metadata, p_actor_id, p_occurred_at,
        p_correlation_id, p_causation_id, p_workspace_id, p_tags
    );
    
    RETURN v_new_version;
END;
$$ LANGUAGE plpgsql;

-- Function to update projection position
CREATE OR REPLACE FUNCTION update_projection_position(
    p_projection_name VARCHAR,
    p_event_id UUID,
    p_position BIGINT
)
RETURNS VOID AS $$
BEGIN
    UPDATE projections
    SET last_processed_event_id = p_event_id,
        last_processed_at = NOW(),
        current_position = p_position,
        updated_at = NOW()
    WHERE projection_name = p_projection_name;
END;
$$ LANGUAGE plpgsql;

-- Function to record projection error
CREATE OR REPLACE FUNCTION record_projection_error(
    p_projection_name VARCHAR,
    p_error TEXT
)
RETURNS VOID AS $$
BEGIN
    UPDATE projections
    SET error_count = error_count + 1,
        last_error = p_error,
        last_error_at = NOW(),
        status = CASE 
            WHEN error_count >= 5 THEN 'ERROR'
            ELSE status
        END,
        updated_at = NOW()
    WHERE projection_name = p_projection_name;
END;
$$ LANGUAGE plpgsql;

-- Create notification trigger for real-time event streaming
CREATE OR REPLACE FUNCTION notify_event()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify(
        'events',
        json_build_object(
            'event_id', NEW.event_id,
            'aggregate_id', NEW.aggregate_id,
            'aggregate_type', NEW.aggregate_type,
            'event_type', NEW.event_type,
            'occurred_at', NEW.occurred_at
        )::text
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER notify_new_event
AFTER INSERT ON events
FOR EACH ROW EXECUTE FUNCTION notify_event();

-- Update trigger for projections
CREATE TRIGGER update_projections_updated_at 
BEFORE UPDATE ON projections
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Update trigger for sagas
CREATE TRIGGER update_sagas_updated_at 
BEFORE UPDATE ON sagas
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Initial projections
INSERT INTO projections (projection_name, projection_type, config) VALUES
    ('entity_count', 'COUNTER', '{"description": "Tracks count of entities by type"}'),
    ('state_distribution', 'AGGREGATOR', '{"description": "Tracks entity state distribution"}'),
    ('recent_activity', 'TIME_SERIES', '{"description": "Recent activity by entity type"}')
ON CONFLICT (projection_name) DO NOTHING;

-- Update metadata
UPDATE storage_metadata 
SET value = '"006"', updated_at = NOW()
WHERE key = 'schema_version';

INSERT INTO storage_metadata (key, value) VALUES
    ('event_sourcing_enabled', 'true')
ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value, updated_at = NOW();

-- Comments for documentation
COMMENT ON TABLE events IS 'Event store for event sourcing pattern';
COMMENT ON TABLE event_snapshots IS 'Aggregate snapshots for performance optimization';
COMMENT ON TABLE projections IS 'Projection state tracking for read models';
COMMENT ON TABLE sagas IS 'Saga/Process manager state for complex workflows';
COMMENT ON TABLE commands IS 'Command queue for CQRS pattern';
COMMENT ON FUNCTION append_event IS 'Append event with optimistic concurrency control';
COMMENT ON FUNCTION get_aggregate_version IS 'Get current version of an aggregate';