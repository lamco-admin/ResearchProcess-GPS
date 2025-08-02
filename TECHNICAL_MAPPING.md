# Technical Mapping: Existing Entities to Meta-Model

### Timestamp: 2025-08-01 23:45:00 EEST

## Entity-by-Entity Mapping

### Layer 1: Core Genealogical Data

#### 1. Source → Entity
```rust
// Current
pub struct Source {
    pub metadata: EntityMetadata,
    pub title: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub repository: Option<String>,
    pub call_number: Option<String>,
    pub source_type: SourceType,
    pub media_type: Option<MediaType>,
    pub abstract_text: Option<String>,
    pub full_text: Option<String>,
    pub url: Option<String>,
    pub accessed_date: Option<DateTime<Utc>>,
    pub created_date: Option<PartialDate>,
    pub modified_date: Option<PartialDate>,
    pub quality_assessment: Option<QualityAssessment>,
    pub citations: Vec<EntityId>,
    pub tags: Vec<String>,
    pub custom_fields: serde_json::Value,
}

// Meta-Model Mapping
Entity {
    entity_type: "Source",
    properties: {
        "title": String,
        "author": String?,
        "publisher": String?,
        "repository": String?,
        "call_number": String?,
        "source_type": String, // Enum → String
        "media_type": String?,
        "abstract_text": String?,
        "full_text": String?,
        "url": String?,
        "accessed_date": DateTime?,
        "created_date": PartialDate?,
        "modified_date": PartialDate?,
        "quality_assessment": PropertyGraph {
            "credibility": f32,
            "relevance": f32,
            "notes": String,
        },
        "tags": Array<String>,
        "custom_fields": PropertyGraph,
    },
    relationships: [
        Relationship {
            relationship_type: "HasCitation",
            to: citation_entity_id,
            direction: "Outgoing",
        }
    ],
    contexts: [
        Context {
            context_type: "Temporal",
            properties: {
                "created": metadata.created_at,
                "modified": metadata.modified_at,
            }
        },
        Context {
            context_type: "Attribution",
            properties: {
                "created_by": metadata.created_by,
                "modified_by": metadata.modified_by,
            }
        }
    ],
}
```

#### 2. Citation → Entity + Relationship
```rust
// Current
pub struct Citation {
    pub metadata: EntityMetadata,
    pub source_id: EntityId,
    pub citation_type: CitationType,
    pub detail: CitationDetail,
    pub accessed_date: Option<DateTime<Utc>>,
    pub evidence: Vec<EntityId>,
    pub quality: Option<CitationQuality>,
    pub transcription: Option<String>,
    pub abstract_text: Option<String>,
    pub notes: Option<String>,
    pub custom_fields: serde_json::Value,
}

// Meta-Model Mapping
Entity {
    entity_type: "Citation",
    properties: {
        "citation_type": String,
        "detail": PropertyGraph { // CitationDetail fields },
        "accessed_date": DateTime?,
        "quality": PropertyGraph { // CitationQuality fields },
        "transcription": String?,
        "abstract_text": String?,
        "notes": String?,
        "custom_fields": PropertyGraph,
    },
    relationships: [
        Relationship {
            relationship_type: "CitesSource",
            to: source_id,
            direction: "Outgoing",
        },
        Relationship {
            relationship_type: "ProvidesEvidence",
            to: evidence_id,
            direction: "Outgoing",
        }
    ],
}
```

#### 3. IdentityPersona → Entity with Complex Relationships
```rust
// Current - complex state machine entity
pub struct IdentityPersona {
    pub metadata: EntityMetadata,
    pub state: PersonaState,
    pub state_history: Vec<StateTransition<PersonaState>>,
    pub identity_type: IdentityType,
    pub primary_name: PersonName,
    pub alternate_names: Vec<PersonName>,
    pub life_events: LifeEvents,
    pub attributes: PersonAttributes,
    pub relationships: Vec<EntityId>,
    pub facts: Vec<EntityId>,
    pub evidence: Vec<EntityId>,
    pub analyses: Vec<EntityId>,
    pub theories: Vec<EntityId>,
    pub certainty: IdentityCertainty,
    pub research_status: ResearchStatus,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub custom_fields: serde_json::Value,
}

// Meta-Model Mapping
Entity {
    entity_type: "IdentityPersona",
    state: "Active", // State machine becomes string state
    properties: {
        "identity_type": String,
        "primary_name": PropertyGraph {
            "given": String,
            "surname": String,
            "prefix": String?,
            "suffix": String?,
            // ... other name fields
        },
        "alternate_names": Array<PropertyGraph>,
        "life_events": PropertyGraph {
            "birth": Event?,
            "death": Event?,
            "burial": Event?,
        },
        "attributes": PropertyGraph {
            "gender": String?,
            "occupation": Array<String>,
            // ... other attributes
        },
        "certainty": PropertyGraph {
            "existence_confidence": f32,
            "identity_confidence": f32,
        },
        "research_status": String,
        "notes": String?,
        "tags": Array<String>,
    },
    relationships: [
        // Family relationships
        Relationship {
            relationship_type: "Parent",
            to: other_persona_id,
            properties: {
                "biological": bool,
                "adoptive": bool,
            }
        },
        // Evidence relationships
        Relationship {
            relationship_type: "SupportedByEvidence",
            to: evidence_id,
        }
    ],
    contexts: [
        Context {
            context_type: "StateHistory",
            properties: {
                "transitions": Array<StateTransition>,
            }
        }
    ],
}
```

### Layer 2: Research Process

#### 4. Theory → Entity with Process State
```rust
// Current
pub struct Theory {
    pub metadata: EntityMetadata,
    pub question: String,
    pub description: Option<String>,
    pub state: TheoryState,
    pub state_history: Vec<StateTransition<TheoryState>>,
    pub parent_theory: Option<EntityId>,
    pub child_theories: Vec<EntityId>,
    pub evidence: Vec<EntityId>,
    pub analyses: Vec<EntityId>,
    pub tags: Vec<String>,
    pub priority: u8,
    pub geographic_scope: Option<GeographicScope>,
    pub temporal_scope: Option<TemporalScope>,
    pub research_log: Vec<ResearchLogEntry>,
    pub conclusion: Option<Conclusion>,
}

// Meta-Model Mapping (Layer 2: Process)
Process {
    process_type: "Theory",
    state: ProcessState::Active, // Maps TheoryState
    properties: {
        "question": String,
        "description": String?,
        "priority": u8,
        "geographic_scope": PropertyGraph?,
        "temporal_scope": PropertyGraph?,
        "conclusion": PropertyGraph?,
        "tags": Array<String>,
    },
    activities: [
        Activity {
            activity_type: "ResearchLogEntry",
            properties: { /* log entry data */ },
        }
    ],
    agents: [researcher_agent_id],
    products: [analysis_report_ids],
}
```

### Layer 3: Configuration & Workflow

#### 5. Workspace → Workspace (Direct Mapping)
```rust
// Current Layer 3 entity maps directly to meta-model Layer 3
Workspace {
    workspace_type: "ResearchProject",
    items: [
        WorkspaceItem::Entities(entity_ids),
        WorkspaceItem::Processes(process_ids),
        WorkspaceItem::Views(view_ids),
    ],
    configuration: Configuration {
        config_type: "MethodologyConfig",
        values: {
            "methodology": "GPS-2021",
            "validation_rules": Array<Rule>,
        }
    },
    governance: Governance {
        governance_type: "Collaborative",
        policies: [...],
        permissions: PermissionSystem { ... },
    },
}
```

## Complex Feature Mappings

### State Machines → Entity State + Context
```rust
// State becomes a property, history goes to context
entity.state = "Active"
entity.contexts.push(Context {
    context_type: "StateHistory",
    properties: {
        "current_state": "Active",
        "transitions": vec![
            {
                "from": "Draft",
                "to": "Active",
                "timestamp": "2025-01-15T10:00:00Z",
                "triggered_by": agent_id,
            }
        ],
        "state_machine_config": state_config,
    }
})
```

### Validation Rules → Context
```rust
entity.contexts.push(Context {
    context_type: "Validation",
    properties: {
        "rules": vec![
            {
                "field": "title",
                "rule": "not_empty",
                "severity": "Error",
            }
        ],
        "last_validated": timestamp,
        "validation_results": results,
    }
})
```

### Collaboration Features → Contexts + Events
```rust
// Real-time collaboration data
entity.contexts.push(Context {
    context_type: "Collaboration",
    properties: {
        "active_editors": vec![agent_id],
        "locks": PropertyGraph,
        "crdt_state": base64_encoded_crdt,
    }
})

// Collaboration tracked through events
Event {
    event_type: "entity.collaborative_edit",
    target: EventTarget::Entity(entity_id),
    data: {
        "changes": crdt_operations,
        "editor": agent_id,
    }
}
```

### Module System Integration
```rust
// Modules work with meta-model entities
impl Module for GenealogyAnalyzer {
    fn handle_message(&mut self, msg: Message) -> Result<Response> {
        match msg {
            Message::AnalyzeEntity { entity_id } => {
                // Fetch meta-model entity
                let entity = self.storage.get_entity(entity_id)?;

                // Work with property graph
                if entity.entity_type == "IdentityPersona" {
                    let name = entity.properties.get("primary_name")?;
                    // ... analysis logic
                }
            }
        }
    }
}
```

## Storage Schema Changes

### Current PostgreSQL Schema
```sql
-- Multiple tables for different entities
CREATE TABLE sources (...);
CREATE TABLE citations (...);
CREATE TABLE identity_personas (...);
```

### Meta-Model PostgreSQL Schema
```sql
-- Unified tables
CREATE TABLE entities (
    id UUID PRIMARY KEY,
    entity_type TEXT NOT NULL,
    state TEXT NOT NULL,
    properties JSONB NOT NULL,
    meta JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE relationships (
    id UUID PRIMARY KEY,
    relationship_type TEXT NOT NULL,
    from_entity UUID REFERENCES entities(id),
    to_entity UUID REFERENCES entities(id),
    direction TEXT NOT NULL,
    properties JSONB NOT NULL,
    certainty JSONB,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE contexts (
    id UUID PRIMARY KEY,
    entity_id UUID REFERENCES entities(id),
    context_type TEXT NOT NULL,
    properties JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

-- Indexes for performance
CREATE INDEX idx_entities_type ON entities(entity_type);
CREATE INDEX idx_entities_properties ON entities USING GIN(properties);
CREATE INDEX idx_relationships_type ON relationships(relationship_type);
CREATE INDEX idx_contexts_entity ON contexts(entity_id);
```

## Direct Migration Scripts

### Entity Migration Functions
```rust
// Direct transformation functions for each entity type
pub fn migrate_source(source: &Source) -> (Entity, Vec<Relationship>) {
    let mut entity = Entity::new("Source");
    entity.id = source.metadata.id.into();
    entity.state = EntityState::Active;

    // Direct property mapping
    entity.properties.set("title", &source.title);
    entity.properties.set("author", &source.author);
    entity.properties.set("publisher", &source.publisher);
    entity.properties.set("repository", &source.repository);
    entity.properties.set("source_type", &source.source_type.to_string());

    // Metadata becomes context
    entity.contexts.push(Context {
        context_type: "Metadata",
        properties: PropertyGraph::from_json(json!({
            "created_by": source.metadata.created_by,
            "created_at": source.metadata.created_at,
            "version": source.metadata.version,
        })),
    });

    // Extract relationships
    let relationships = source.citations.iter().map(|cit_id| {
        Relationship::new("HasCitation", entity.id, (*cit_id).into())
    }).collect();

    (entity, relationships)
}
```

### Batch Migration Script
```rust
// Complete migration in one transaction
pub async fn migrate_database(old_db: &PgPool, new_db: &PgPool) -> Result<()> {
    let tx = new_db.begin().await?;

    // Migrate all entities
    migrate_all_sources(&tx, old_db).await?;
    migrate_all_citations(&tx, old_db).await?;
    migrate_all_personas(&tx, old_db).await?;
    // ... all 18 entity types

    // Verify integrity
    verify_migration(&tx, old_db).await?;

    tx.commit().await?;
    Ok(())
}
```

## Benefits of Direct Migration

1. **Clean Cut**: No compatibility layer complexity
2. **Single Source of Truth**: Only meta-model after migration
3. **Simplified Architecture**: One data model throughout
4. **Performance**: No dual-model overhead
5. **Clear Timeline**: Defined migration window

## Migration Execution Plan

1. **Week 1-3**: Port all infrastructure to meta-model
2. **Week 4**: Create and test migration scripts
3. **Week 5**: Full backup of production data
4. **Week 6**: Execute migration during maintenance window
5. **Week 7-8**: Monitor and optimize performance

## Post-Migration Architecture

After migration, the system will have:
- Single universal data model (Entity, Relationship, Context, Certainty)
- All features working directly with meta-model
- No legacy code or compatibility layers
- Clean, extensible architecture ready for new features