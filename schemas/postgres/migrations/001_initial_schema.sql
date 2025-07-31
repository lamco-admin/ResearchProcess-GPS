-- ResearchProcess-GPS PostgreSQL Schema
-- Version: 001_initial_schema
-- Description: Core tables for hybrid JSONB + binary storage

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "btree_gin";  -- For indexing JSONB

-- Workspaces for multi-tenancy support
CREATE TABLE workspaces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    owner_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    settings JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    
    CONSTRAINT uk_workspace_name UNIQUE (name)
);

-- Core entities table (all entities stored here)
CREATE TABLE entities (
    -- Core fields
    id UUID PRIMARY KEY,
    entity_type VARCHAR(100) NOT NULL,
    workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE,
    
    -- JSONB data storage (flexible schema)
    data JSONB NOT NULL,
    
    -- Binary data (for efficient protocol messages)
    binary_data BYTEA,
    
    -- Metadata
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    version BIGINT NOT NULL DEFAULT 1,
    
    -- State tracking (for entities with state machines)
    current_state VARCHAR(100),
    state_changed_at TIMESTAMPTZ,
    
    -- Soft delete support
    deleted_at TIMESTAMPTZ,
    deleted_by UUID,
    
    -- Indexes will be added below
    CONSTRAINT chk_version CHECK (version > 0)
);

-- Entity version history
CREATE TABLE entity_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL,
    version BIGINT NOT NULL,
    data JSONB NOT NULL,
    binary_data BYTEA,
    changed_by UUID NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    change_reason TEXT,
    parent_version BIGINT,
    
    CONSTRAINT fk_entity_versions_entity FOREIGN KEY (entity_id) 
        REFERENCES entities(id) ON DELETE CASCADE,
    CONSTRAINT uk_entity_version UNIQUE (entity_id, version)
);

-- State machine transitions
CREATE TABLE entity_states (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL,
    from_state VARCHAR(100),
    to_state VARCHAR(100) NOT NULL,
    transitioned_by UUID NOT NULL,
    transitioned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reason TEXT,
    metadata JSONB,
    
    CONSTRAINT fk_entity_states_entity FOREIGN KEY (entity_id) 
        REFERENCES entities(id) ON DELETE CASCADE
);

-- Relationships between entities
CREATE TABLE relationships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE,
    from_entity UUID NOT NULL,
    to_entity UUID NOT NULL,
    relationship_type VARCHAR(100) NOT NULL,
    properties JSONB NOT NULL DEFAULT '{}',
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    
    CONSTRAINT fk_relationships_from FOREIGN KEY (from_entity) 
        REFERENCES entities(id) ON DELETE CASCADE,
    CONSTRAINT fk_relationships_to FOREIGN KEY (to_entity) 
        REFERENCES entities(id) ON DELETE CASCADE,
    CONSTRAINT chk_different_entities CHECK (from_entity != to_entity)
);

-- Change data capture
CREATE TABLE changes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE,
    operation VARCHAR(20) NOT NULL CHECK (operation IN ('CREATE', 'UPDATE', 'DELETE', 'STATE_TRANSITION')),
    changed_by UUID NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    before_data JSONB,
    after_data JSONB,
    metadata JSONB,
    
    -- Partitioning by month might be added later
    CONSTRAINT fk_changes_entity FOREIGN KEY (entity_id) 
        REFERENCES entities(id) ON DELETE CASCADE
);

-- Module registry
CREATE TABLE modules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    version VARCHAR(50) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    config JSONB NOT NULL DEFAULT '{}',
    installed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    installed_by UUID NOT NULL,
    
    CONSTRAINT uk_module_name UNIQUE (name)
);

-- Storage metadata
CREATE TABLE storage_metadata (
    key VARCHAR(255) PRIMARY KEY,
    value JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
-- Entity type and workspace queries
CREATE INDEX idx_entities_type ON entities(entity_type) WHERE deleted_at IS NULL;
CREATE INDEX idx_entities_workspace ON entities(workspace_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_entities_created_by ON entities(created_by) WHERE deleted_at IS NULL;
CREATE INDEX idx_entities_state ON entities(current_state) WHERE current_state IS NOT NULL AND deleted_at IS NULL;

-- JSONB indexes for common queries
CREATE INDEX idx_entities_data ON entities USING GIN (data);

-- Specific JSONB path indexes (examples - add more based on usage)
CREATE INDEX idx_entities_researcher_name ON entities ((data->>'name')) 
    WHERE entity_type = 'Researcher' AND deleted_at IS NULL;
CREATE INDEX idx_entities_theory_question ON entities ((data->>'question')) 
    WHERE entity_type = 'Theory' AND deleted_at IS NULL;
CREATE INDEX idx_entities_evidence_type ON entities ((data->>'evidence_type')) 
    WHERE entity_type = 'Evidence' AND deleted_at IS NULL;

-- Relationship indexes
CREATE INDEX idx_relationships_from ON relationships(from_entity) WHERE deleted_at IS NULL;
CREATE INDEX idx_relationships_to ON relationships(to_entity) WHERE deleted_at IS NULL;
CREATE INDEX idx_relationships_type ON relationships(relationship_type) WHERE deleted_at IS NULL;

-- Version history indexes
CREATE INDEX idx_entity_versions_entity ON entity_versions(entity_id);
CREATE INDEX idx_entity_versions_changed_at ON entity_versions(changed_at);

-- Change tracking indexes
CREATE INDEX idx_changes_entity ON changes(entity_id);
CREATE INDEX idx_changes_changed_at ON changes(changed_at);
CREATE INDEX idx_changes_operation ON changes(operation);

-- Function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to automatically update updated_at
CREATE TRIGGER update_entities_updated_at BEFORE UPDATE ON entities
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Function to track changes
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
        COALESCE(NEW.updated_at, NOW()), -- Use updated_at as proxy for changed_by
        v_before,
        v_after
    );
    
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger for change tracking
CREATE TRIGGER track_changes AFTER INSERT OR UPDATE OR DELETE ON entities
    FOR EACH ROW EXECUTE FUNCTION track_entity_changes();

-- Function to increment version on update
CREATE OR REPLACE FUNCTION increment_version()
RETURNS TRIGGER AS $$
BEGIN
    NEW.version = OLD.version + 1;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to increment version
CREATE TRIGGER increment_entity_version BEFORE UPDATE ON entities
    FOR EACH ROW EXECUTE FUNCTION increment_version();

-- Materialized view for entity statistics (refresh periodically)
CREATE MATERIALIZED VIEW entity_statistics AS
SELECT 
    entity_type,
    COUNT(*) as count,
    COUNT(DISTINCT workspace_id) as workspace_count,
    MIN(created_at) as first_created,
    MAX(created_at) as last_created,
    MAX(updated_at) as last_updated
FROM entities
WHERE deleted_at IS NULL
GROUP BY entity_type;

-- Index for statistics
CREATE UNIQUE INDEX idx_entity_statistics_type ON entity_statistics(entity_type);

-- Initial metadata
INSERT INTO storage_metadata (key, value) VALUES
    ('schema_version', '"001"'),
    ('migration_date', to_jsonb(NOW())),
    ('features', '{"vector_search": false, "graph_operations": false}');

-- Comments for documentation
COMMENT ON TABLE entities IS 'Core entity storage with hybrid JSONB + binary data';
COMMENT ON COLUMN entities.data IS 'Flexible JSONB storage for entity-specific data';
COMMENT ON COLUMN entities.binary_data IS 'Efficient binary storage for protocol messages';
COMMENT ON TABLE entity_versions IS 'Complete version history for all entities';
COMMENT ON TABLE relationships IS 'Graph-like relationships between entities';
COMMENT ON TABLE changes IS 'Change data capture for sync and audit';