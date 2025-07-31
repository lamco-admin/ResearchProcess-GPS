-- ResearchProcess-GPS Domain-Specific Optimizations
-- Version: 004_domain_specific
-- Description: Indexes and functions specific to genealogical research

-- Theory-specific indexes
CREATE INDEX idx_theory_state ON entities ((data->>'state')) 
    WHERE entity_type = 'Theory' AND deleted_at IS NULL;
CREATE INDEX idx_theory_created_date ON entities ((data->>'created_at')) 
    WHERE entity_type = 'Theory' AND deleted_at IS NULL;

-- Evidence-specific indexes
CREATE INDEX idx_evidence_quality ON entities ((data->'quality')) 
    WHERE entity_type = 'Evidence' AND deleted_at IS NULL;
CREATE INDEX idx_evidence_source ON entities ((data->>'source_id')) 
    WHERE entity_type = 'Evidence' AND deleted_at IS NULL;

-- IdentityPersona-specific indexes
CREATE INDEX idx_identity_state ON entities ((data->>'state')) 
    WHERE entity_type = 'IdentityPersona' AND deleted_at IS NULL;
CREATE INDEX idx_identity_evidence_refs ON entities USING GIN ((data->'evidence_references')) 
    WHERE entity_type = 'IdentityPersona' AND deleted_at IS NULL;

-- Person-specific indexes (concluded identities)
CREATE INDEX idx_person_birth_year ON entities ((data->'birth'->>'year')) 
    WHERE entity_type = 'Person' AND deleted_at IS NULL;
CREATE INDEX idx_person_death_year ON entities ((data->'death'->>'year')) 
    WHERE entity_type = 'Person' AND deleted_at IS NULL;

-- Location-specific indexes
CREATE INDEX idx_location_coordinates ON entities ((data->'coordinates')) 
    WHERE entity_type = 'Location' AND deleted_at IS NULL;
CREATE INDEX idx_location_jurisdiction ON entities ((data->>'jurisdiction')) 
    WHERE entity_type = 'Location' AND deleted_at IS NULL;

-- Event-specific indexes
CREATE INDEX idx_event_date ON entities ((data->'temporal_data'->>'date')) 
    WHERE entity_type = 'Event' AND deleted_at IS NULL;
CREATE INDEX idx_event_type ON entities ((data->>'event_type')) 
    WHERE entity_type = 'Event' AND deleted_at IS NULL;

-- Analysis-specific indexes
CREATE INDEX idx_analysis_type ON entities ((data->>'analysis_type')) 
    WHERE entity_type = 'Analysis' AND deleted_at IS NULL;
CREATE INDEX idx_analysis_confidence ON entities ((data->>'overall_confidence')) 
    WHERE entity_type = 'Analysis' AND deleted_at IS NULL;

-- Helper function to find all evidence for a theory
CREATE OR REPLACE FUNCTION find_theory_evidence(p_theory_id UUID)
RETURNS TABLE (
    evidence_id UUID,
    evidence_type TEXT,
    quality JSONB,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        e.id,
        e.data->>'evidence_type',
        e.data->'quality',
        e.created_at
    FROM entities e
    WHERE e.entity_type = 'Evidence'
    AND e.deleted_at IS NULL
    AND e.data @> jsonb_build_object('theory_id', p_theory_id::text);
END;
$$ LANGUAGE plpgsql;

-- Helper function to find all personas for a person
CREATE OR REPLACE FUNCTION find_person_personas(p_person_id UUID)
RETURNS TABLE (
    persona_id UUID,
    state TEXT,
    evidence_count INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        e.id,
        e.data->>'state',
        jsonb_array_length(e.data->'evidence_references')
    FROM entities e
    WHERE e.entity_type = 'IdentityPersona'
    AND e.deleted_at IS NULL
    AND e.data @> jsonb_build_object('person_id', p_person_id::text);
END;
$$ LANGUAGE plpgsql;

-- Function to calculate research coverage for a theory
CREATE OR REPLACE FUNCTION calculate_research_coverage(p_theory_id UUID)
RETURNS JSONB AS $$
DECLARE
    coverage JSONB;
    evidence_count INTEGER;
    source_count INTEGER;
    analysis_count INTEGER;
BEGIN
    -- Count evidence
    SELECT COUNT(DISTINCT e.id) INTO evidence_count
    FROM entities e
    WHERE e.entity_type = 'Evidence'
    AND e.deleted_at IS NULL
    AND e.data @> jsonb_build_object('theory_id', p_theory_id::text);
    
    -- Count unique sources
    SELECT COUNT(DISTINCT e.data->>'source_id') INTO source_count
    FROM entities e
    WHERE e.entity_type = 'Evidence'
    AND e.deleted_at IS NULL
    AND e.data @> jsonb_build_object('theory_id', p_theory_id::text);
    
    -- Count analyses
    SELECT COUNT(DISTINCT e.id) INTO analysis_count
    FROM entities e
    WHERE e.entity_type = 'Analysis'
    AND e.deleted_at IS NULL
    AND e.data->'scope'->'entities' @> to_jsonb(ARRAY[p_theory_id]);
    
    coverage := jsonb_build_object(
        'evidence_count', evidence_count,
        'source_count', source_count,
        'analysis_count', analysis_count,
        'calculated_at', NOW()
    );
    
    RETURN coverage;
END;
$$ LANGUAGE plpgsql;

-- Function to find potential duplicate persons
CREATE OR REPLACE FUNCTION find_potential_duplicates(
    p_name TEXT,
    p_birth_year INTEGER,
    p_threshold NUMERIC DEFAULT 0.8
)
RETURNS TABLE (
    person_id UUID,
    similarity_score NUMERIC,
    name TEXT,
    birth_year INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        e.id,
        similarity(e.data->>'name', p_name) as sim_score,
        e.data->>'name',
        (e.data->'birth'->>'year')::INTEGER
    FROM entities e
    WHERE e.entity_type = 'Person'
    AND e.deleted_at IS NULL
    AND similarity(e.data->>'name', p_name) > p_threshold
    AND ABS((e.data->'birth'->>'year')::INTEGER - p_birth_year) <= 5
    ORDER BY sim_score DESC;
END;
$$ LANGUAGE plpgsql;

-- Materialized view for theory progress tracking
CREATE MATERIALIZED VIEW theory_progress AS
SELECT 
    t.id as theory_id,
    t.data->>'question' as question,
    t.data->>'state' as state,
    t.created_at,
    t.updated_at,
    COUNT(DISTINCT e.id) as evidence_count,
    COUNT(DISTINCT a.id) as analysis_count,
    MAX(e.updated_at) as last_evidence_added,
    MAX(a.updated_at) as last_analysis_added
FROM entities t
LEFT JOIN entities e ON 
    e.entity_type = 'Evidence' 
    AND e.data @> jsonb_build_object('theory_id', t.id::text)
    AND e.deleted_at IS NULL
LEFT JOIN entities a ON 
    a.entity_type = 'Analysis'
    AND a.data->'scope'->'entities' @> to_jsonb(ARRAY[t.id])
    AND a.deleted_at IS NULL
WHERE t.entity_type = 'Theory'
AND t.deleted_at IS NULL
GROUP BY t.id, t.data->>'question', t.data->>'state', t.created_at, t.updated_at;

CREATE UNIQUE INDEX idx_theory_progress ON theory_progress(theory_id);

-- Function to refresh progress view
CREATE OR REPLACE FUNCTION refresh_theory_progress()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY theory_progress;
END;
$$ LANGUAGE plpgsql;

-- Trigger function for maintaining relationships graph
CREATE OR REPLACE FUNCTION maintain_relationship_graph()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
        -- If entity has parent reference, create relationship
        IF NEW.data ? 'parent_id' AND NEW.data->>'parent_id' IS NOT NULL THEN
            INSERT INTO relationships (
                from_entity, to_entity, relationship_type,
                properties, created_by, workspace_id
            ) VALUES (
                (NEW.data->>'parent_id')::UUID,
                NEW.id,
                'parent_of',
                jsonb_build_object('auto_created', true),
                NEW.created_by,
                NEW.workspace_id
            ) ON CONFLICT DO NOTHING;
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Enable relationship auto-creation for hierarchical entities
CREATE TRIGGER auto_create_relationships
    AFTER INSERT OR UPDATE ON entities
    FOR EACH ROW
    WHEN (NEW.data ? 'parent_id')
    EXECUTE FUNCTION maintain_relationship_graph();

-- Performance hint: GPS compliance tracking
CREATE OR REPLACE VIEW gps_compliance_summary AS
SELECT 
    t.id as theory_id,
    t.data->>'question' as question,
    c.data->'gps_elements' as gps_elements,
    c.data->'coverage' as research_coverage,
    c.updated_at as last_assessed
FROM entities t
LEFT JOIN entities c ON 
    c.entity_type = 'Confidence'
    AND c.data @> jsonb_build_object('theory_id', t.id::text)
    AND c.deleted_at IS NULL
WHERE t.entity_type = 'Theory'
AND t.deleted_at IS NULL;

-- Update metadata
UPDATE storage_metadata 
SET value = value || '{"domain_optimizations": true}'::jsonb 
WHERE key = 'features';

COMMENT ON FUNCTION find_theory_evidence IS 'Find all evidence associated with a theory';
COMMENT ON FUNCTION find_potential_duplicates IS 'Find possible duplicate person records using fuzzy matching';
COMMENT ON MATERIALIZED VIEW theory_progress IS 'Cached view of theory research progress';