# Developer Guide: Building Adapters and Plugins

## Overview

This guide shows you how to extend ResearchProcess-GPS by creating:

- **Adapters**: Import/export data from other genealogy systems
- **Plugins**: Add custom functionality to the platform
- **Schemas**: Define custom data models
- **UI Components**: Extend the web interface

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                   Web Interface (Svelte)                 │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │  Workspace  │  │SchemaManager │  │EntityBrowser │  │
│  └─────────────┘  └──────────────┘  └──────────────┘  │
└────────────────────────┬────────────────────────────────┘
                         │ WASM Bindings (JavaScript ↔ Rust)
┌────────────────────────▼────────────────────────────────┐
│              rp-wasm (WASM Bridge)                       │
│  ┌───────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ │
│  │WasmEntity │ │WasmSchema│ │ WasmRel  │ │WasmStorage│ │
│  └───────────┘ └──────────┘ └──────────┘ └──────────┘ │
└────────────────────────┬────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────┐
│            Core Rust Crates                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │rp-meta-core  │  │  rp-schema   │  │  rp-storage  │ │
│  │(Universal    │  │(Schema &     │  │(IndexedDB)   │ │
│  │ Meta-Model)  │  │ Validation)  │  │              │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────┘
```

## Part 1: Creating an Adapter

Adapters import and export data from other systems. Let's create a GEDCOM importer.

### Step 1: Define Your Adapter

```rust
// crates/rp-adapters/src/gedcom_adapter.rs

use async_trait::async_trait;
use rp_meta_core::{Entity, Relationship};
use rp_schema::{Schema, Adapter, ImportResult, AdapterInfo};
use anyhow::Result;

pub struct GedcomAdapter {
    schema: Schema,
}

impl GedcomAdapter {
    pub fn new() -> Result<Self> {
        // Load GEDCOM schema
        let schema_yaml = include_str!("../../../schemas/gedcom-7.yaml");
        let schema = Schema::from_yaml(schema_yaml)?;

        Ok(Self { schema })
    }
}

#[async_trait]
impl Adapter for GedcomAdapter {
    fn id(&self) -> &str {
        "gedcom-7-adapter"
    }

    fn info(&self) -> AdapterInfo {
        AdapterInfo {
            name: "GEDCOM 7.0 Adapter".to_string(),
            version: "1.0.0".to_string(),
            description: "Import and export GEDCOM 7.0 files".to_string(),
            supported_formats: vec!["ged".to_string(), "gedcom".to_string()],
            capabilities: vec!["import".to_string(), "export".to_string()],
        }
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }

    async fn import(&self, data: &[u8]) -> Result<ImportResult> {
        // Parse GEDCOM file
        let gedcom_str = std::str::from_utf8(data)?;
        let records = parse_gedcom(gedcom_str)?;

        let mut entities = Vec::new();
        let mut relationships = Vec::new();

        // Convert GEDCOM records to universal entities
        for record in records {
            match record.tag.as_str() {
                "INDI" => {
                    let entity = self.convert_individual(&record)?;
                    entities.push(entity);
                }
                "FAM" => {
                    let (family_entity, rels) = self.convert_family(&record)?;
                    entities.push(family_entity);
                    relationships.extend(rels);
                }
                "SOUR" => {
                    let entity = self.convert_source(&record)?;
                    entities.push(entity);
                }
                _ => {
                    // Skip unknown tags or log warning
                }
            }
        }

        Ok(ImportResult {
            entities,
            relationships,
            warnings: vec![],
            stats: Default::default(),
        })
    }

    async fn export(
        &self,
        entities: &[Entity],
        relationships: &[Relationship]
    ) -> Result<Vec<u8>> {
        let mut gedcom = String::new();

        // Write header
        gedcom.push_str("0 HEAD\n");
        gedcom.push_str("1 GEDC\n");
        gedcom.push_str("2 VERS 7.0\n");

        // Export entities
        for entity in entities {
            match entity.entity_type.as_str() {
                "Individual" => {
                    gedcom.push_str(&self.export_individual(entity)?);
                }
                "Family" => {
                    gedcom.push_str(&self.export_family(entity, relationships)?);
                }
                "Source" => {
                    gedcom.push_str(&self.export_source(entity)?);
                }
                _ => {
                    // Unknown entity type, skip or warn
                }
            }
        }

        // Write trailer
        gedcom.push_str("0 TRLR\n");

        Ok(gedcom.into_bytes())
    }
}

impl GedcomAdapter {
    fn convert_individual(&self, record: &GedcomRecord) -> Result<Entity> {
        let mut entity = Entity::new("Individual");

        // Extract name
        if let Some(name) = record.get_tag("NAME") {
            entity.set_property("name", Property::Value(Value::Text(name.value.clone())));
        }

        // Extract sex
        if let Some(sex) = record.get_tag("SEX") {
            entity.set_property("sex", Property::Value(Value::Text(sex.value.clone())));
        }

        // Extract birth
        if let Some(birth) = record.get_tag("BIRT") {
            if let Some(date) = birth.get_tag("DATE") {
                let temporal = parse_gedcom_date(&date.value)?;
                entity.set_property("birth_date", Property::Value(Value::Temporal(temporal)));
            }
            if let Some(place) = birth.get_tag("PLAC") {
                let spatial = SpatialValue::from_text(&place.value);
                entity.set_property("birth_place", Property::Value(Value::Spatial(spatial)));
            }
        }

        // Continue for other tags...

        Ok(entity)
    }

    fn convert_family(&self, record: &GedcomRecord) -> Result<(Entity, Vec<Relationship>)> {
        let mut family_entity = Entity::new("Family");
        let mut relationships = Vec::new();

        // Extract marriage info
        if let Some(marr) = record.get_tag("MARR") {
            if let Some(date) = marr.get_tag("DATE") {
                let temporal = parse_gedcom_date(&date.value)?;
                family_entity.set_property("marriage_date", Property::Value(Value::Temporal(temporal)));
            }
        }

        // Create parent-child relationships
        for child in record.get_all_tags("CHIL") {
            let mut rel = Relationship::new("Parent-Child");

            // Add parents
            if let Some(husb) = record.get_tag("HUSB") {
                let parent_id = EntityId::parse(&husb.value)?;
                rel.add_participant(Participant::new(parent_id, "parent"));
            }
            if let Some(wife) = record.get_tag("WIFE") {
                let parent_id = EntityId::parse(&wife.value)?;
                rel.add_participant(Participant::new(parent_id, "parent"));
            }

            // Add child
            let child_id = EntityId::parse(&child.value)?;
            rel.add_participant(Participant::new(child_id, "child"));

            relationships.push(rel);
        }

        Ok((family_entity, relationships))
    }

    fn export_individual(&self, entity: &Entity) -> Result<String> {
        let mut lines = Vec::new();

        lines.push(format!("0 @{}@ INDI", entity.id));

        // Export name
        if let Some(name) = entity.get_property("name") {
            if let Property::Value(Value::Text(text)) = name {
                lines.push(format!("1 NAME {}", text));
            }
        }

        // Export sex
        if let Some(sex) = entity.get_property("sex") {
            if let Property::Value(Value::Text(text)) = sex {
                lines.push(format!("1 SEX {}", text));
            }
        }

        // Export birth
        if entity.get_property("birth_date").is_some() || entity.get_property("birth_place").is_some() {
            lines.push("1 BIRT".to_string());

            if let Some(Property::Value(Value::Temporal(temporal))) = entity.get_property("birth_date") {
                lines.push(format!("2 DATE {}", format_gedcom_date(temporal)));
            }

            if let Some(Property::Value(Value::Spatial(spatial))) = entity.get_property("birth_place") {
                lines.push(format!("2 PLAC {}", spatial.to_string()));
            }
        }

        // Continue for other properties...

        Ok(lines.join("\n") + "\n")
    }

    // Additional helper methods...
}
```

### Step 2: Register Your Adapter

```rust
// In your adapter registry

use rp_schema::AdapterRegistry;

let mut registry = AdapterRegistry::new();

// Register GEDCOM adapter
registry.register(Box::new(GedcomAdapter::new()?));

// Register GRAMPS adapter
registry.register(Box::new(GrampsAdapter::new()?));

// List available adapters
for adapter_id in registry.list() {
    let adapter = registry.get(adapter_id)?;
    println!("{}: {}", adapter.id(), adapter.info().name);
}
```

### Step 3: Use Your Adapter

```rust
// Import from GEDCOM
let adapter = registry.get("gedcom-7-adapter")?;
let file_data = std::fs::read("family.ged")?;
let result = adapter.import(&file_data).await?;

// Add to workspace
for entity in result.entities {
    workspace.add_entity(&entity)?;
}
for relationship in result.relationships {
    workspace.add_relationship(&relationship)?;
}

// Export to GEDCOM
let all_entities = workspace.get_all_entities();
let all_relationships = workspace.get_all_relationships();
let gedcom_data = adapter.export(&all_entities, &all_relationships).await?;
std::fs::write("export.ged", gedcom_data)?;
```

## Part 2: Creating a Schema

Schemas define your data model. Here's how to create a custom schema:

### Basic Schema Structure

```yaml
# schemas/my-research-model.yaml

schema:
  id: my-research
  version: "1.0.0"
  name: My Research Model
  description: Custom model for historical research

entity_types:
  # Define your entity types
  Document:
    description: A historical document
    required_properties:
      - title
      - date
    optional_properties:
      - author
      - repository
      - transcription
    properties:
      title:
        type: Text
        description: Document title
        validation:
          - rule: Required
      date:
        type: Temporal
        description: Document date
      author:
        type: Text
        description: Author or creator
      repository:
        type: Reference
        description: Where document is held
      transcription:
        type: Text
        description: Full text transcription
    valid_states:
      - Draft
      - Transcribed
      - Verified
      - Published
    default_state: Draft

  Hypothesis:
    description: A research hypothesis
    required_properties:
      - statement
    optional_properties:
      - evidence_for
      - evidence_against
      - confidence
      - notes
    properties:
      statement:
        type: Text
        validation:
          - rule: Required
      evidence_for:
        type: Collection
        item_type: Reference
      evidence_against:
        type: Collection
        item_type: Reference
      confidence:
        type: Enum
        values: [Very Low, Low, Medium, High, Very High]
      notes:
        type: Text
    valid_states:
      - Proposed
      - Under Review
      - Supported
      - Refuted
      - Abandoned
    default_state: Proposed

relationship_types:
  Cites:
    description: Document cites another document
    participants:
      - role: citing
        entity_types: [Document]
        required: true
      - role: cited
        entity_types: [Document]
        required: true
    optional_properties:
      - page
      - quote
    properties:
      page:
        type: Text
      quote:
        type: Text
    min_participants: 2
    max_participants: 2

  Supports:
    description: Evidence supports hypothesis
    participants:
      - role: evidence
        entity_types: [Document]
        required: true
      - role: hypothesis
        entity_types: [Hypothesis]
        required: true
    optional_properties:
      - strength
    properties:
      strength:
        type: Enum
        values: [Weak, Moderate, Strong]
    min_participants: 2
    max_participants: 2
```

### Advanced Schema Features

#### Complex Property Types

```yaml
properties:
  # Nested map
  address:
    type: Map
    properties:
      street:
        type: Text
      city:
        type: Text
      state:
        type: Text
      zip:
        type: Text
        pattern: "^\\d{5}(-\\d{4})?$"

  # Collection with constraints
  children:
    type: Collection
    item_type: Reference
    min_items: 0
    max_items: 20

  # Integer with range
  age:
    type: Integer
    min: 0
    max: 150

  # Text with length constraints
  bio:
    type: Text
    min_length: 10
    max_length: 5000

  # Text with pattern
  phone:
    type: Text
    pattern: "^[+]?[0-9\\s\\-()]+$"
```

#### State Machines

```yaml
state_machines:
  document_workflow:
    initial_state: Draft
    states:
      - Draft
      - Transcribed
      - Reviewed
      - Verified
      - Published
      - Archived
    transitions:
      Draft: [Transcribed]
      Transcribed: [Draft, Reviewed]
      Reviewed: [Transcribed, Verified]
      Verified: [Published, Reviewed]
      Published: [Archived]
      Archived: []
    terminal_states:
      - Archived
```

#### Validation Rules

```yaml
properties:
  name:
    type: Text
    validation:
      - rule: Required
      - rule: MinLength
        value: 2
      - rule: MaxLength
        value: 100
      - rule: Pattern
        value: "^[A-Za-z\\s]+$"

  email:
    type: Text
    validation:
      - rule: Required
      - rule: Email

  age:
    type: Integer
    validation:
      - rule: Min
        value: 0
      - rule: Max
        value: 150
```

## Part 3: Extending the Web Interface

### Adding a Custom Component

```svelte
<!-- web/src/components/MyCustomComponent.svelte -->

<script>
  export let workspace
  export let wasmModule

  let myData = []

  async function loadData() {
    // Use WASM API
    const entities = workspace.get_entities_by_type("Document");
    myData = entities;
  }

  async function processData() {
    // Create entities
    const entity = workspace.create_entity("Document");
    entity.set_text("title", "My Document");

    // Add to workspace
    await workspace.add_entity(entity);

    // Reload
    await loadData();
  }

  // Load on mount
  import { onMount } from 'svelte';
  onMount(loadData);
</script>

<style>
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  .card {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    margin-bottom: 1rem;
  }

  button {
    padding: 0.75rem 1.5rem;
    background: #3498db;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  button:hover {
    background: #2980b9;
  }
</style>

<div class="container">
  <h2>My Custom Component</h2>

  <button on:click={processData}>
    Process Data
  </button>

  {#each myData as item}
    <div class="card">
      <pre>{JSON.stringify(item, null, 2)}</pre>
    </div>
  {/each}
</div>
```

### Integrating into App

```svelte
<!-- web/src/App.svelte -->

<script>
  import MyCustomComponent from './components/MyCustomComponent.svelte'

  // Add to tabs
  let activeTab = 'workspace'  // Add 'custom' option
</script>

<nav>
  <!-- Add custom tab -->
  <button
    class="tab"
    class:active={activeTab === 'custom'}
    on:click={() => activeTab = 'custom'}
  >
    My Feature
  </button>
</nav>

<main>
  {#if activeTab === 'custom'}
    <MyCustomComponent {workspace} {wasmModule} />
  {/if}
</main>
```

## Part 4: Testing Your Extensions

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gedcom_import() {
        let adapter = GedcomAdapter::new().unwrap();

        let gedcom = r#"
0 HEAD
1 GEDC
2 VERS 7.0
0 @I1@ INDI
1 NAME John /Smith/
1 SEX M
1 BIRT
2 DATE 15 MAR 1850
2 PLAC New York, NY
0 TRLR
"#;

        let result = adapter.import(gedcom.as_bytes()).await.unwrap();

        assert_eq!(result.entities.len(), 1);

        let person = &result.entities[0];
        assert_eq!(person.entity_type, "Individual");

        let name = person.get_property("name").unwrap();
        assert!(matches!(name, Property::Value(Value::Text(s)) if s == "John /Smith/"));
    }

    #[tokio::test]
    async fn test_gedcom_roundtrip() {
        let adapter = GedcomAdapter::new().unwrap();

        // Import
        let original = std::fs::read("test_data/family.ged").unwrap();
        let import_result = adapter.import(&original).await.unwrap();

        // Export
        let exported = adapter.export(
            &import_result.entities,
            &import_result.relationships
        ).await.unwrap();

        // Re-import
        let reimport_result = adapter.import(&exported).await.unwrap();

        // Verify counts match
        assert_eq!(import_result.entities.len(), reimport_result.entities.len());
        assert_eq!(import_result.relationships.len(), reimport_result.relationships.len());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_full_workflow() {
    // Create workspace
    let mut workspace = Workspace::new("Test").await.unwrap();

    // Load schema
    let schema_yaml = include_str!("../schemas/gedcom-7.yaml");
    workspace.load_schema_yaml(schema_yaml).unwrap();
    workspace.set_active_schema("gedcom-7").unwrap();

    // Import data
    let adapter = GedcomAdapter::new().unwrap();
    let gedcom_data = std::fs::read("test_data/family.ged").unwrap();
    let result = adapter.import(&gedcom_data).await.unwrap();

    // Add to workspace
    for entity in result.entities {
        workspace.add_entity(&entity).unwrap();
    }

    // Verify
    assert_eq!(workspace.entity_count(), expected_count);

    // Export
    let all_entities = workspace.get_all_entities();
    let exported = adapter.export(&all_entities, &[]).await.unwrap();

    // Verify exported format
    let exported_str = String::from_utf8(exported).unwrap();
    assert!(exported_str.contains("0 HEAD"));
    assert!(exported_str.contains("0 TRLR"));
}
```

## Part 5: Best Practices

### Error Handling

Follow the NO_FALLBACK_POLICY:

```rust
// ❌ BAD: Silent fallback
let name = entity.get_property("name").unwrap_or_default();

// ✅ GOOD: Explicit error
let name = entity.get_property("name")
    .ok_or_else(|| Error::PropertyNotFound("name".to_string()))?;

// ❌ BAD: Swallow error
if let Ok(value) = parse_date(&s) {
    return value;
}
return default_date();

// ✅ GOOD: Propagate error
let value = parse_date(&s)
    .map_err(|e| Error::DateParseFailed(s.to_string(), e))?;
```

### Performance

- Use `Entity::new()` for bulk creation
- Batch database operations
- Use indexes for lookups
- Validate before persisting

### Documentation

Document your adapter:

```rust
/// GEDCOM 7.0 Adapter
///
/// Imports and exports GEDCOM 7.0 format files.
///
/// # Features
///
/// - Full GEDCOM 7.0 specification support
/// - Preserves custom tags
/// - Handles multimedia references
/// - Supports LDS ordinances
///
/// # Example
///
/// ```rust
/// let adapter = GedcomAdapter::new()?;
/// let data = std::fs::read("family.ged")?;
/// let result = adapter.import(&data).await?;
/// ```
```

## Resources

- [Schema Guide](SCHEMA_GUIDE.md) - Complete schema reference
- [API Documentation](../api/) - Rust API reference
- [Examples](../../examples/) - Example adapters and plugins
- [Test Data](../../test_data/) - Sample files for testing

## Get Help

- Open an issue on GitHub
- Join discussions
- Check existing adapters for reference

Happy coding!
