-- ResearchProcess-GPS Fix Event Sourcing Function
-- Version: 008_fix_append_event
-- Description: Fix append_event function to avoid FOR UPDATE with aggregate functions

-- Drop and recreate the append_event function with the fix
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

-- Update metadata
UPDATE storage_metadata 
SET value = '"008"', updated_at = NOW()
WHERE key = 'schema_version';

-- Add comment about the fix
COMMENT ON FUNCTION append_event IS 'Append event with optimistic concurrency control - Fixed to avoid FOR UPDATE with aggregate functions';