-- ResearchProcess-GPS Enhanced Event Notification
-- Version: 009_enhance_notify
-- Description: Update notify_event function to include more event data

-- Drop and recreate the notify_event function with enhanced data
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
            'occurred_at', NEW.occurred_at,
            'event_data', NEW.event_data,
            'actor_id', NEW.actor_id,
            'version', NEW.aggregate_version
        )::text
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Update metadata
UPDATE storage_metadata 
SET value = '"009"', updated_at = NOW()
WHERE key = 'schema_version';

-- Add comment about the enhancement
COMMENT ON FUNCTION notify_event IS 'Notify with full event data for real-time streaming';