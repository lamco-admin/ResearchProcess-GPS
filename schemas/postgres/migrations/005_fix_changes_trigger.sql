-- Fix the change tracking trigger to use proper changed_by value
-- Version: 005_fix_changes_trigger
-- Description: Fix changed_by field in change tracking trigger

-- Drop the existing trigger
DROP TRIGGER IF EXISTS track_changes ON entities;

-- Drop the existing function
DROP FUNCTION IF EXISTS track_entity_changes();

-- Create corrected function to track changes
CREATE OR REPLACE FUNCTION track_entity_changes()
RETURNS TRIGGER AS $$
DECLARE
    v_operation VARCHAR(20);
    v_before JSONB;
    v_after JSONB;
BEGIN
    IF TG_OP = 'INSERT' THEN
        v_operation := 'CREATE';
        v_before := NULL;
        v_after := NEW.data;
    ELSIF TG_OP = 'UPDATE' THEN
        -- Check if it's a state transition
        IF OLD.current_state IS DISTINCT FROM NEW.current_state THEN
            v_operation := 'STATE_TRANSITION';
        ELSE
            v_operation := 'UPDATE';
        END IF;
        v_before := OLD.data;
        v_after := NEW.data;
    ELSIF TG_OP = 'DELETE' THEN
        v_operation := 'DELETE';
        v_before := OLD.data;
        v_after := NULL;
    END IF;
    
    -- Insert change record
    INSERT INTO changes (
        entity_id, entity_type, workspace_id, operation,
        changed_by, before_data, after_data
    ) VALUES (
        COALESCE(NEW.id, OLD.id),
        COALESCE(NEW.entity_type, OLD.entity_type),
        COALESCE(NEW.workspace_id, OLD.workspace_id),
        v_operation,
        COALESCE(NEW.created_by, OLD.created_by), -- Use created_by as changed_by
        v_before,
        v_after
    );
    
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Recreate trigger for change tracking
CREATE TRIGGER track_changes AFTER INSERT OR UPDATE OR DELETE ON entities
    FOR EACH ROW EXECUTE FUNCTION track_entity_changes();

-- Update metadata
UPDATE storage_metadata 
SET value = '"005"'
WHERE key = 'schema_version';

COMMENT ON FUNCTION track_entity_changes IS 'Track all changes to entities with corrected changed_by field';