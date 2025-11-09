-- Person Authority Control System
-- Version: 010_person_authority_control
-- Description: Canonical person records based on Koha authority control patterns
--
-- This implements Person as a canonical entity (authority record) separate from
-- IdentityPersona (research workflow). An IdentityPersona in "Concluded" state
-- links to a canonical Person entity.
--
-- Based on research from: KOHA_AUTHORITY_ARCHITECTURE_STUDY_2025_11_09_1535_UTC.md
-- Design spec: RP_GPS_PERSON_ENTITY_DESIGN_2025_11_09_1535_UTC.md

-- =============================================================================
-- CUSTOM TYPES
-- =============================================================================

-- Sex/Gender enumeration
CREATE TYPE person_sex AS ENUM ('Male', 'Female', 'Unknown');

-- Variant name types
CREATE TYPE variant_name_type AS ENUM (
    'birth',          -- Name at birth
    'married',        -- After marriage
    'divorced',       -- After divorce (maiden name restored)
    'nickname',       -- Common nickname
    'immigration',    -- Name change on immigration
    'spelling',       -- Spelling variation
    'translation',    -- Language translation
    'abbreviation',   -- Abbreviated form
    'pseudonym',      -- Pen name or alias
    'legal',          -- Legal name change
    'religious',      -- Religious/monastic name
    'stage',          -- Stage/professional name
    'documented'      -- Exactly as appears in specific source
);

-- Relationship types (directional: person_1 → person_2)
CREATE TYPE person_relationship_type AS ENUM (
    'parent',
    'child',
    'spouse',
    'sibling',
    'grandparent',
    'grandchild',
    'aunt_uncle',
    'niece_nephew',
    'cousin',
    'step_parent',
    'step_child',
    'adoptive_parent',
    'adoptive_child',
    'foster_parent',
    'foster_child',
    'guardian',
    'ward'
);

-- Confidence levels (1-5 scale)
CREATE TYPE person_confidence AS ENUM (
    'speculative',    -- 1
    'uncertain',      -- 2
    'possible',       -- 3
    'probable',       -- 4
    'definite'        -- 5
);

-- Date certainty types
CREATE TYPE date_certainty AS ENUM (
    'exact',
    'estimated',
    'calculated',
    'before',
    'after',
    'between'
);

-- =============================================================================
-- MAIN PERSONS TABLE (Canonical Authority Records)
-- =============================================================================

CREATE TABLE persons (
    -- Primary Key
    person_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Canonical Name Components (established form)
    given_name VARCHAR(255),
    surname VARCHAR(255),
    middle_name VARCHAR(255),
    name_prefix VARCHAR(50),     -- Dr., Rev., Hon., etc.
    name_suffix VARCHAR(50),     -- Jr., III, Esq., etc.

    -- Vital Events - Birth
    birth_date_year INT,
    birth_date_month SMALLINT CHECK (birth_date_month BETWEEN 1 AND 12),
    birth_date_day SMALLINT CHECK (birth_date_day BETWEEN 1 AND 31),
    birth_date_certainty date_certainty,
    birth_date_circa BOOLEAN DEFAULT FALSE,
    birth_date_end_year INT,     -- For "between" ranges
    birth_date_original_text VARCHAR(200),
    birth_place_id UUID,         -- References places table (future)
    birth_source_id UUID,        -- References sources table (future)

    -- Vital Events - Death
    death_date_year INT,
    death_date_month SMALLINT CHECK (death_date_month BETWEEN 1 AND 12),
    death_date_day SMALLINT CHECK (death_date_day BETWEEN 1 AND 31),
    death_date_certainty date_certainty,
    death_date_circa BOOLEAN DEFAULT FALSE,
    death_date_end_year INT,
    death_date_original_text VARCHAR(200),
    death_place_id UUID,
    death_source_id UUID,

    -- Biographical
    sex person_sex DEFAULT 'Unknown',
    occupation VARCHAR(500),
    religion VARCHAR(100),

    -- Research Notes
    notes TEXT,
    research_notes TEXT,         -- Private notes for active research
    conclusion_notes TEXT,       -- Why this canonical form was chosen

    -- Confidence in this conclusion
    conclusion_confidence person_confidence,

    -- MARC Export Cache (for library system interoperability)
    marc_binary BYTEA,           -- Binary MARC21 authority
    marc_xml TEXT,               -- XML MARC21 authority
    marc_updated_at TIMESTAMPTZ,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL,    -- References users/researchers
    last_modified_by UUID NOT NULL,

    -- Archival (for merged persons - soft delete)
    archived BOOLEAN DEFAULT FALSE,
    archived_reason TEXT,
    archived_at TIMESTAMPTZ,
    archived_by UUID,

    -- Constraints
    CONSTRAINT chk_person_has_name CHECK (
        given_name IS NOT NULL OR surname IS NOT NULL
    ),
    CONSTRAINT chk_birth_date_range CHECK (
        birth_date_certainty != 'between' OR birth_date_end_year IS NOT NULL
    ),
    CONSTRAINT chk_death_date_range CHECK (
        death_date_certainty != 'between' OR death_date_end_year IS NOT NULL
    )
);

-- Indexes for Persons
CREATE INDEX idx_persons_surname ON persons(surname) WHERE NOT archived;
CREATE INDEX idx_persons_given_name ON persons(given_name) WHERE NOT archived;
CREATE INDEX idx_persons_full_name ON persons((given_name || ' ' || surname)) WHERE NOT archived;
CREATE INDEX idx_persons_birth_year ON persons(birth_date_year) WHERE NOT archived;
CREATE INDEX idx_persons_death_year ON persons(death_date_year) WHERE NOT archived;
CREATE INDEX idx_persons_updated_at ON persons(updated_at);
CREATE INDEX idx_persons_created_by ON persons(created_by);
CREATE INDEX idx_persons_archived ON persons(archived) WHERE archived = TRUE;

-- Full-text search on persons
CREATE INDEX idx_persons_search ON persons
USING GIN(to_tsvector('english',
    COALESCE(given_name, '') || ' ' ||
    COALESCE(surname, '') || ' ' ||
    COALESCE(notes, '')
)) WHERE NOT archived;

COMMENT ON TABLE persons IS 'Canonical person records (authority control) - one per individual. Based on Koha authority patterns.';
COMMENT ON COLUMN persons.person_id IS 'Unique identifier for this canonical person';
COMMENT ON COLUMN persons.birth_date_certainty IS 'Certainty type: exact, estimated, calculated, before, after, between';
COMMENT ON COLUMN persons.conclusion_confidence IS 'Overall confidence in this canonical person conclusion (1=speculative, 5=definite)';
COMMENT ON COLUMN persons.marc_binary IS 'Cached MARC21 authority record (binary) for library system interoperability';
COMMENT ON COLUMN persons.archived IS 'TRUE if person was merged or deleted (soft delete)';

-- =============================================================================
-- PERSON VARIANT NAMES (All name forms)
-- =============================================================================

CREATE TABLE person_variant_names (
    variant_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_id UUID NOT NULL REFERENCES persons(person_id) ON DELETE CASCADE,

    -- Variant Name Components
    given_name VARCHAR(255),
    surname VARCHAR(255),
    middle_name VARCHAR(255),
    name_prefix VARCHAR(50),
    name_suffix VARCHAR(50),
    full_name VARCHAR(500) NOT NULL,    -- Searchable combined form

    -- Variant Classification
    variant_type variant_name_type NOT NULL,
    language VARCHAR(10),                -- ISO 639-1 code: 'en', 'de', 'pl', etc.

    -- Attribution: Where was this variant found?
    source_id UUID,                      -- References sources (future)
    source_citation TEXT,                -- Specific page/line
    notes TEXT,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL,

    CONSTRAINT chk_variant_has_name CHECK (
        given_name IS NOT NULL OR surname IS NOT NULL
    )
);

-- Indexes for Variant Names
CREATE INDEX idx_variant_names_person ON person_variant_names(person_id);
CREATE INDEX idx_variant_names_full ON person_variant_names(full_name);
CREATE INDEX idx_variant_names_surname ON person_variant_names(surname);
CREATE INDEX idx_variant_names_type ON person_variant_names(variant_type);
CREATE INDEX idx_variant_names_source ON person_variant_names(source_id);

-- Full-text search on variants
CREATE INDEX idx_variant_names_search ON person_variant_names
USING GIN(to_tsvector('english', full_name || ' ' || COALESCE(notes, '')));

-- Prevent exact duplicates
CREATE UNIQUE INDEX idx_variant_unique ON person_variant_names(
    person_id,
    LOWER(full_name),  -- Case-insensitive
    variant_type
);

COMMENT ON TABLE person_variant_names IS 'All name variants for canonical persons (like MARC 4XX fields)';
COMMENT ON COLUMN person_variant_names.variant_type IS 'Type: birth, married, nickname, immigration, spelling, etc.';
COMMENT ON COLUMN person_variant_names.source_id IS 'Source where this variant was encountered';

-- =============================================================================
-- PERSON RELATIONSHIPS (Family structure)
-- =============================================================================

CREATE TABLE person_relationships (
    relationship_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- The two persons involved (directional)
    person_1_id UUID NOT NULL REFERENCES persons(person_id),
    person_2_id UUID NOT NULL REFERENCES persons(person_id),

    -- Relationship type (directional: person_1 → person_2)
    relationship_type person_relationship_type NOT NULL,

    -- Evidence & Certainty
    certainty person_confidence NOT NULL,
    source_id UUID,              -- References sources (future)
    notes TEXT,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by UUID NOT NULL,

    CONSTRAINT chk_different_persons CHECK (person_1_id != person_2_id)
);

-- Indexes for Relationships
CREATE INDEX idx_relationships_person1 ON person_relationships(person_1_id);
CREATE INDEX idx_relationships_person2 ON person_relationships(person_2_id);
CREATE INDEX idx_relationships_type ON person_relationships(relationship_type);
CREATE INDEX idx_relationships_certainty ON person_relationships(certainty);

-- Composite index for finding specific relationship between two persons
CREATE INDEX idx_relationships_pair ON person_relationships(
    person_1_id,
    person_2_id,
    relationship_type
);

COMMENT ON TABLE person_relationships IS 'Relationships between canonical persons (like MARC 5XX fields)';
COMMENT ON COLUMN person_relationships.relationship_type IS 'Directional: parent, child, spouse, sibling, etc.';
COMMENT ON COLUMN person_relationships.certainty IS 'Confidence: 1=speculative, 2=uncertain, 3=possible, 4=probable, 5=definite';

-- =============================================================================
-- SOURCE PERSONS (Source mentions/citations linking to persons)
-- =============================================================================

CREATE TABLE source_persons (
    source_person_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Source this mention appears in
    source_id UUID NOT NULL,     -- References sources (future: FK)

    -- Canonical person (NULL if not yet linked)
    person_id UUID REFERENCES persons(person_id) ON DELETE SET NULL,

    -- Name exactly as it appears in source
    name_in_source VARCHAR(500) NOT NULL,

    -- Role in this source
    role VARCHAR(100),           -- 'subject', 'author', 'witness', 'informant', etc.

    -- Additional extracted data
    age_in_source INT CHECK (age_in_source >= 0 AND age_in_source <= 150),
    relationship_in_source VARCHAR(200),  -- "son of", "wife of", etc.
    occupation_in_source VARCHAR(200),
    residence_in_source VARCHAR(500),

    -- Linking metadata (when linked to canonical person)
    certainty person_confidence,
    linked_at TIMESTAMPTZ,
    linked_by UUID,
    linking_notes TEXT,

    -- Extraction metadata
    page_number VARCHAR(50),
    line_number VARCHAR(50),
    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL
);

-- Indexes for Source Persons
CREATE INDEX idx_source_persons_source ON source_persons(source_id);
CREATE INDEX idx_source_persons_person ON source_persons(person_id);
CREATE INDEX idx_source_persons_name ON source_persons(name_in_source);
CREATE INDEX idx_source_persons_role ON source_persons(role);

-- Find unlinked mentions
CREATE INDEX idx_source_persons_unlinked ON source_persons(source_id)
    WHERE person_id IS NULL;

-- Full-text search on source mentions
CREATE INDEX idx_source_persons_search ON source_persons
USING GIN(to_tsvector('english', name_in_source || ' ' || COALESCE(notes, '')));

COMMENT ON TABLE source_persons IS 'Person mentions in sources (analogous to Koha biblio field → authority link via MARC $9)';
COMMENT ON COLUMN source_persons.person_id IS 'Link to canonical person (like MARC $9 subfield in Koha)';
COMMENT ON COLUMN source_persons.name_in_source IS 'Exact name as written in source (may differ from canonical)';
COMMENT ON COLUMN source_persons.certainty IS 'Confidence that this mention = canonical person';

-- =============================================================================
-- PERSON MERGES (Audit log for merge operations)
-- =============================================================================

CREATE TABLE person_merges (
    merge_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- The merge operation
    from_person_id UUID NOT NULL,  -- Archived person (duplicate)
    to_person_id UUID NOT NULL REFERENCES persons(person_id),  -- Canonical

    -- Strategy used
    merge_strategy VARCHAR(50) NOT NULL,  -- 'keep_all', 'prefer_canonical', 'prefer_source', 'manual'

    -- Execution
    performed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    performed_by UUID NOT NULL,

    -- Reversibility
    reversible BOOLEAN DEFAULT TRUE,
    reversed_at TIMESTAMPTZ,
    reversed_by UUID,

    -- Data backup (for reversal) - complete Person record as JSON
    from_person_backup JSONB,

    notes TEXT
);

-- Indexes for Merges
CREATE INDEX idx_merges_from ON person_merges(from_person_id);
CREATE INDEX idx_merges_to ON person_merges(to_person_id);
CREATE INDEX idx_merges_performed_at ON person_merges(performed_at);
CREATE INDEX idx_merges_reversible ON person_merges(reversible)
    WHERE reversed_at IS NULL AND reversible = TRUE;

COMMENT ON TABLE person_merges IS 'Log of person merge operations (for audit and potential reversal)';
COMMENT ON COLUMN person_merges.from_person_backup IS 'JSON snapshot of archived person (enables reversal)';
COMMENT ON COLUMN person_merges.merge_strategy IS 'How conflicts were resolved: keep_all, prefer_canonical, prefer_source, manual';

-- =============================================================================
-- HELPER FUNCTIONS
-- =============================================================================

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to build full name from components
CREATE OR REPLACE FUNCTION build_full_person_name(
    p_given VARCHAR,
    p_surname VARCHAR,
    p_middle VARCHAR DEFAULT NULL,
    p_prefix VARCHAR DEFAULT NULL,
    p_suffix VARCHAR DEFAULT NULL
) RETURNS VARCHAR AS $$
DECLARE
    parts TEXT[];
    full_name TEXT;
BEGIN
    parts := ARRAY[]::TEXT[];

    IF p_prefix IS NOT NULL THEN
        parts := array_append(parts, p_prefix);
    END IF;
    IF p_given IS NOT NULL THEN
        parts := array_append(parts, p_given);
    END IF;
    IF p_middle IS NOT NULL THEN
        parts := array_append(parts, p_middle);
    END IF;
    IF p_surname IS NOT NULL THEN
        parts := array_append(parts, p_surname);
    END IF;
    IF p_suffix IS NOT NULL THEN
        parts := array_append(parts, p_suffix);
    END IF;

    full_name := array_to_string(parts, ' ');
    RETURN full_name;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Function to get person's canonical name
CREATE OR REPLACE FUNCTION get_person_canonical_name(p_person_id UUID)
RETURNS VARCHAR AS $$
DECLARE
    v_name VARCHAR;
BEGIN
    SELECT build_full_person_name(given_name, surname, middle_name, name_prefix, name_suffix)
    INTO v_name
    FROM persons
    WHERE person_id = p_person_id;

    RETURN COALESCE(v_name, 'Unknown Person');
END;
$$ LANGUAGE plpgsql STABLE;

-- Function to get person's life span string
CREATE OR REPLACE FUNCTION get_person_life_span(p_person_id UUID)
RETURNS VARCHAR AS $$
DECLARE
    v_birth VARCHAR;
    v_death VARCHAR;
    v_circa_prefix VARCHAR;
BEGIN
    SELECT
        CASE
            WHEN birth_date_circa THEN 'ca. '
            WHEN birth_date_certainty = 'estimated' THEN 'est. '
            WHEN birth_date_certainty = 'calculated' THEN 'calc. '
            WHEN birth_date_certainty = 'before' THEN 'bef. '
            WHEN birth_date_certainty = 'after' THEN 'aft. '
            ELSE ''
        END || birth_date_year::TEXT,
        CASE
            WHEN death_date_circa THEN 'ca. '
            WHEN death_date_certainty = 'estimated' THEN 'est. '
            WHEN death_date_certainty = 'calculated' THEN 'calc. '
            WHEN death_date_certainty = 'before' THEN 'bef. '
            WHEN death_date_certainty = 'after' THEN 'aft. '
            ELSE ''
        END || death_date_year::TEXT
    INTO v_birth, v_death
    FROM persons
    WHERE person_id = p_person_id;

    IF v_birth IS NOT NULL AND v_death IS NOT NULL THEN
        RETURN v_birth || ' - ' || v_death;
    ELSIF v_birth IS NOT NULL THEN
        RETURN v_birth || ' - ';
    ELSIF v_death IS NOT NULL THEN
        RETURN ' - ' || v_death;
    ELSE
        RETURN NULL;
    END IF;
END;
$$ LANGUAGE plpgsql STABLE;

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Auto-update updated_at on persons
CREATE TRIGGER update_persons_updated_at
BEFORE UPDATE ON persons
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Auto-update updated_at on person_relationships
CREATE TRIGGER update_relationships_updated_at
BEFORE UPDATE ON person_relationships
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Auto-populate full_name on person_variant_names insert/update
CREATE OR REPLACE FUNCTION populate_variant_full_name()
RETURNS TRIGGER AS $$
BEGIN
    NEW.full_name := build_full_person_name(
        NEW.given_name,
        NEW.surname,
        NEW.middle_name,
        NEW.name_prefix,
        NEW.name_suffix
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER populate_variant_names_full_name
BEFORE INSERT OR UPDATE ON person_variant_names
FOR EACH ROW
WHEN (NEW.given_name IS NOT NULL OR NEW.surname IS NOT NULL)
EXECUTE FUNCTION populate_variant_full_name();

-- =============================================================================
-- VIEWS FOR CONVENIENT QUERIES
-- =============================================================================

-- View of active (non-archived) persons with computed fields
CREATE VIEW v_active_persons AS
SELECT
    p.*,
    build_full_person_name(p.given_name, p.surname, p.middle_name, p.name_prefix, p.name_suffix) AS canonical_name,
    get_person_life_span(p.person_id) AS life_span,
    (SELECT COUNT(*) FROM person_variant_names v WHERE v.person_id = p.person_id) AS variant_count,
    (SELECT COUNT(*) FROM person_relationships r WHERE r.person_1_id = p.person_id OR r.person_2_id = p.person_id) AS relationship_count,
    (SELECT COUNT(*) FROM source_persons sp WHERE sp.person_id = p.person_id) AS source_citation_count
FROM persons p
WHERE NOT p.archived;

COMMENT ON VIEW v_active_persons IS 'Active persons with computed canonical name, life span, and counts';

-- View of persons with all their variants (for search)
CREATE VIEW v_persons_with_variants AS
SELECT
    p.person_id,
    p.given_name AS canonical_given,
    p.surname AS canonical_surname,
    build_full_person_name(p.given_name, p.surname, p.middle_name, p.name_prefix, p.name_suffix) AS canonical_name,
    p.birth_date_year,
    p.death_date_year,
    p.sex,
    array_agg(DISTINCT v.full_name ORDER BY v.full_name) FILTER (WHERE v.full_name IS NOT NULL) AS variant_names,
    array_agg(DISTINCT v.variant_type ORDER BY v.variant_type) FILTER (WHERE v.variant_type IS NOT NULL) AS variant_types
FROM persons p
LEFT JOIN person_variant_names v ON v.person_id = p.person_id
WHERE NOT p.archived
GROUP BY p.person_id;

COMMENT ON VIEW v_persons_with_variants IS 'Persons with aggregated variant names for comprehensive search';

-- =============================================================================
-- SAMPLE DATA (Optional - for testing)
-- =============================================================================

-- Example canonical person
-- INSERT INTO persons (
--     person_id, given_name, surname, birth_date_year, birth_date_certainty,
--     death_date_year, death_date_certainty, sex, conclusion_confidence,
--     notes, created_by, last_modified_by
-- ) VALUES (
--     '550e8400-e29b-41d4-a716-446655440000',
--     'John', 'Smith', 1820, 'exact', 1891, 'exact', 'Male', 'definite',
--     'Immigrant from Germany, settled in Ohio',
--     '550e8400-e29b-41d4-a716-446655440001',  -- researcher ID
--     '550e8400-e29b-41d4-a716-446655440001'
-- );

-- Example variant names
-- INSERT INTO person_variant_names (person_id, given_name, surname, variant_type, created_by)
-- VALUES
--     ('550e8400-e29b-41d4-a716-446655440000', 'Johann', 'Schmidt', 'immigration', '550e8400-e29b-41d4-a716-446655440001'),
--     ('550e8400-e29b-41d4-a716-446655440000', 'John', 'Schmidt', 'spelling', '550e8400-e29b-41d4-a716-446655440001'),
--     ('550e8400-e29b-41d4-a716-446655440000', 'J.', 'Smith', 'abbreviation', '550e8400-e29b-41d4-a716-446655440001');

-- =============================================================================
-- PERMISSIONS (Adjust as needed)
-- =============================================================================

-- Grant permissions to application role (adjust role name as needed)
-- GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO rp_app;
-- GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO rp_app;
-- GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO rp_app;

-- =============================================================================
-- END OF MIGRATION
-- =============================================================================
