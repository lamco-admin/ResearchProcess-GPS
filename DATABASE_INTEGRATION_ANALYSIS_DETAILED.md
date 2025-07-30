# Detailed Database Integration Analysis for Research-GPS

## Current Database Schema Analysis

### Overview
The genealogy_data_models database contains comprehensive information about genealogy systems, their features, and entity types. This analysis examines how current systems handle research features and identifies integration points.

## Current Research Feature Support

### Systems with Research Tools
From the feature_matrix analysis:

| System | Research Tools | Evidence Pattern |
|--------|----------------|------------------|
| Family Tree Maker | Research Logs | FTM Container Model |
| Legacy Family Tree | Research Logs | Standard Two-tier |
| WikiTree | Research Logs | Collaborative Inline Model |
| RootsMagic | Research Logs | Two-tier Source Model |
| Reunion | Research Logs | Shared Source Model |
| MacFamilyTree | Research Logs | Advanced Place Model |
| GEDCOM | None | Academic Source Model |
| Ancestry | None | Simple Model |

### Key Findings
1. **Limited Research Support**: Only 6 of 18 major systems have any research tools
2. **Research Logs Only**: All systems that support research only offer basic research logs
3. **No Advanced Features**: No evidence of hypothesis tracking, proof statements, or GPS tools
4. **Evidence Models Vary**: Different approaches to evidence handling across systems

## Entity Type Analysis

### Research-Related Entities Found
Only 3 research-related entity types exist across all systems:

1. **EvidenceReference** (GEDCOM X)
   - Modern evidence-based approach
   - Supports evidence/conclusion separation

2. **RESEARCHLOG** (RootsMagic)
   - Basic research logging
   - Proprietary implementation

3. **_LOG** (Family Tree Maker)
   - Research log implementation
   - Uses underscore prefix (custom tag)

### Missing Entity Types
No systems currently implement:
- Hypothesis tracking
- Proof statements/arguments
- Evidence analysis structures
- Research plans
- GPS compliance tracking
- Task management for research

## Database Structure for Integration

### Core Tables Relevant for Integration

#### 1. systems
- Tracks all genealogy software and standards
- system_type: 'software', 'standard', 'methodology', 'service'
- Key for identifying integration targets

#### 2. entity_types
- Defines data structures per system version
- Links to system_versions for versioning
- Includes semantic_vector for AI similarity
- internal_structure (JSONB) for detailed schemas

#### 3. feature_matrix
- Comprehensive feature tracking
- research_tools column shows current support
- evidence_pattern shows evidence handling approach
- Links to both systems and versions

#### 4. entity_relationships
- Tracks relationships between entities
- Important for mapping research entities to persons/sources

## Integration Architecture

### Phase 1: Extension Tables
```sql
-- Research activity tracking
CREATE TABLE research_activities (
    activity_id SERIAL PRIMARY KEY,
    system_id INTEGER REFERENCES systems(system_id),
    activity_date DATE NOT NULL,
    repository_name VARCHAR(255),
    search_description TEXT,
    results_summary TEXT,
    negative_result BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Link activities to existing entities
CREATE TABLE activity_entity_links (
    link_id SERIAL PRIMARY KEY,
    activity_id INTEGER REFERENCES research_activities(activity_id),
    entity_type VARCHAR(50), -- 'person', 'source', etc.
    entity_reference VARCHAR(255), -- System-specific ID
    relationship_type VARCHAR(50) -- 'searched_for', 'found', etc.
);

-- Research plans
CREATE TABLE research_plans (
    plan_id SERIAL PRIMARY KEY,
    system_id INTEGER REFERENCES systems(system_id),
    research_question TEXT NOT NULL,
    methodology TEXT,
    status VARCHAR(20) DEFAULT 'active',
    created_date DATE DEFAULT CURRENT_DATE,
    completed_date DATE
);

-- Hypotheses tracking
CREATE TABLE hypotheses (
    hypothesis_id SERIAL PRIMARY KEY,
    plan_id INTEGER REFERENCES research_plans(plan_id),
    description TEXT NOT NULL,
    status VARCHAR(20) DEFAULT 'proposed',
    confidence_level INTEGER CHECK (confidence_level BETWEEN 1 AND 5),
    test_results TEXT
);
```

### Phase 2: System-Specific Mappings
```sql
-- Map research entities to system-specific structures
CREATE TABLE system_research_mappings (
    mapping_id SERIAL PRIMARY KEY,
    system_id INTEGER REFERENCES systems(system_id),
    research_entity_type VARCHAR(50), -- 'research_log', 'hypothesis', etc.
    system_entity_code VARCHAR(50), -- e.g., '_LOG', 'RESEARCHLOG'
    mapping_rules JSONB, -- Transformation rules
    FOREIGN KEY (system_id, system_entity_code) 
        REFERENCES entity_types(version_id, entity_code)
);

-- Evidence quality assessments
CREATE TABLE evidence_assessments (
    assessment_id SERIAL PRIMARY KEY,
    system_id INTEGER REFERENCES systems(system_id),
    source_reference VARCHAR(255),
    information_quality VARCHAR(20),
    evidence_type VARCHAR(20),
    relevance_score INTEGER,
    analysis_notes TEXT,
    gps_criteria JSONB
);
```

### Phase 3: Cross-System Synchronization
```sql
-- Track research data across systems
CREATE TABLE research_sync_log (
    sync_id SERIAL PRIMARY KEY,
    from_system_id INTEGER REFERENCES systems(system_id),
    to_system_id INTEGER REFERENCES systems(system_id),
    entity_type VARCHAR(50),
    sync_status VARCHAR(20),
    sync_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    transformation_log JSONB
);
```

## Integration Strategies by System Type

### 1. Systems with Research Logs (FTM, RootsMagic, etc.)
**Strategy**: Enhance existing research logs
```sql
-- Extend research log functionality
CREATE VIEW enhanced_research_logs AS
SELECT 
    rl.*,
    ra.activity_date,
    ra.negative_result,
    ea.evidence_type,
    ea.gps_criteria
FROM system_research_logs rl
LEFT JOIN research_activities ra ON rl.log_id = ra.external_id
LEFT JOIN evidence_assessments ea ON rl.source_id = ea.source_reference;
```

**Integration Points**:
- Map _LOG or RESEARCHLOG entities
- Enhance with GPS compliance fields
- Add hypothesis linking
- Enable evidence quality tracking

### 2. Systems without Research Support (GEDCOM, Ancestry)
**Strategy**: Parallel research database
```sql
-- Create shadow research tables
CREATE TABLE research_shadows (
    shadow_id SERIAL PRIMARY KEY,
    system_id INTEGER REFERENCES systems(system_id),
    person_identifier VARCHAR(255),
    research_status VARCHAR(20),
    last_research_date DATE,
    research_completeness INTEGER,
    notes TEXT
);
```

**Integration Points**:
- Use NOTE fields for basic storage
- External research database
- ID mapping for synchronization
- Import/export via enhanced GEDCOM

### 3. Evidence-Based Systems (GEDCOM X)
**Strategy**: Native extension
```sql
-- Leverage existing evidence model
CREATE TABLE gedcomx_research_extensions (
    extension_id SERIAL PRIMARY KEY,
    evidence_reference_id VARCHAR(255),
    research_activity_id INTEGER REFERENCES research_activities(activity_id),
    proof_statement TEXT,
    gps_compliance JSONB
);
```

**Integration Points**:
- Extend EvidenceReference
- Add research workflow
- Native GPS support
- Full evidence chain

## Performance Considerations

### Indexing Strategy
```sql
-- Research activity performance
CREATE INDEX idx_research_date ON research_activities(activity_date DESC);
CREATE INDEX idx_research_system ON research_activities(system_id);
CREATE INDEX idx_activity_links ON activity_entity_links(entity_reference);

-- Evidence assessment performance
CREATE INDEX idx_evidence_source ON evidence_assessments(source_reference);
CREATE INDEX idx_evidence_quality ON evidence_assessments(information_quality);

-- Full-text search on research
CREATE INDEX idx_research_search ON research_activities 
    USING gin(to_tsvector('english', search_description || ' ' || results_summary));
```

### Query Optimization
```sql
-- Efficient research timeline query
CREATE MATERIALIZED VIEW research_timeline AS
SELECT 
    ra.activity_date,
    s.system_name,
    ra.repository_name,
    ra.search_description,
    array_agg(ael.entity_reference) as related_entities
FROM research_activities ra
JOIN systems s ON ra.system_id = s.system_id
LEFT JOIN activity_entity_links ael ON ra.activity_id = ael.activity_id
GROUP BY ra.activity_id, s.system_name
ORDER BY ra.activity_date DESC;

-- Refresh periodically
CREATE INDEX idx_timeline_date ON research_timeline(activity_date);
```

## Migration Path

### Step 1: System Analysis
```sql
-- Identify systems needing migration
SELECT 
    s.system_name,
    s.vendor,
    fm.research_tools,
    fm.evidence_pattern,
    COUNT(DISTINCT e.entity_code) as entity_count
FROM systems s
LEFT JOIN feature_matrix fm ON s.system_id = fm.system_id
LEFT JOIN system_versions sv ON s.system_id = sv.system_id
LEFT JOIN entity_types e ON sv.version_id = e.version_id
WHERE s.system_type = 'software'
GROUP BY s.system_id, s.system_name, s.vendor, fm.research_tools, fm.evidence_pattern
ORDER BY 
    CASE WHEN fm.research_tools IS NOT NULL THEN 0 ELSE 1 END,
    entity_count DESC;
```

### Step 2: Data Mapping
```sql
-- Create mapping templates
INSERT INTO system_research_mappings (system_id, research_entity_type, system_entity_code, mapping_rules)
SELECT 
    s.system_id,
    'research_log',
    e.entity_code,
    jsonb_build_object(
        'source_field', 'log_entry',
        'date_field', 'log_date',
        'transform', 'direct'
    )
FROM systems s
JOIN system_versions sv ON s.system_id = sv.system_id
JOIN entity_types e ON sv.version_id = e.version_id
WHERE e.entity_code IN ('_LOG', 'RESEARCHLOG');
```

### Step 3: Migration Execution
```sql
-- Batch migration process
CREATE OR REPLACE FUNCTION migrate_research_logs(p_system_id INTEGER)
RETURNS INTEGER AS $$
DECLARE
    v_count INTEGER := 0;
BEGIN
    INSERT INTO research_activities (
        system_id,
        activity_date,
        search_description,
        results_summary
    )
    SELECT 
        p_system_id,
        COALESCE(log_date, CURRENT_DATE),
        log_description,
        log_results
    FROM legacy_research_logs
    WHERE system_id = p_system_id
    ON CONFLICT DO NOTHING;
    
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$ LANGUAGE plpgsql;
```

## Success Metrics

### Integration Completeness
```sql
-- Monitor integration progress
CREATE VIEW integration_metrics AS
SELECT 
    s.system_name,
    COUNT(DISTINCT ra.activity_id) as research_activities,
    COUNT(DISTINCT rp.plan_id) as research_plans,
    COUNT(DISTINCT h.hypothesis_id) as hypotheses,
    COUNT(DISTINCT ea.assessment_id) as evidence_assessments,
    MAX(ra.activity_date) as last_activity
FROM systems s
LEFT JOIN research_activities ra ON s.system_id = ra.system_id
LEFT JOIN research_plans rp ON s.system_id = rp.system_id
LEFT JOIN hypotheses h ON rp.plan_id = h.plan_id
LEFT JOIN evidence_assessments ea ON s.system_id = ea.system_id
WHERE s.system_type = 'software'
GROUP BY s.system_id, s.system_name
ORDER BY research_activities DESC;
```

## Recommendations

1. **Start with High-Adoption Systems**: Focus on Family Tree Maker and RootsMagic first
2. **Leverage Existing Research Logs**: Build on what users already know
3. **Create Migration Tools**: Automated conversion from basic logs to enhanced research
4. **Maintain Backward Compatibility**: Ensure existing workflows continue to function
5. **Provide Clear Value**: Show GPS compliance benefits immediately
6. **Build Community**: Engage professional genealogists for testing and feedback

## Conclusion

The database analysis reveals minimal current support for research features across genealogy systems. Only basic research logs exist in 6 of 18 major systems. This presents both a challenge and opportunity for Research-GPS implementation. The proposed integration architecture provides a path to enhance existing systems while maintaining compatibility and enabling new research workflow capabilities.