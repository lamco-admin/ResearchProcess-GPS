//! WASM Workspace - main entry point for working with data

use wasm_bindgen::prelude::*;
use std::collections::HashMap;

use rp_meta_core::{Entity, Relationship, EntityId};
use rp_schema::SchemaRegistry;

use crate::entity::WasmEntity;
use crate::relationship::WasmRelationship;
use crate::schema::WasmSchema;
use crate::utils::to_js_error;

/// Main workspace for managing entities, relationships, and schemas
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmWorkspace {
    name: String,
    entities: HashMap<String, Entity>,
    relationships: HashMap<String, Relationship>,
    schema_registry: SchemaRegistry,
    active_schema: Option<String>,
}

#[wasm_bindgen]
impl WasmWorkspace {
    /// Create a new workspace
    pub async fn new(name: String) -> Result<WasmWorkspace, JsValue> {
        Ok(WasmWorkspace {
            name,
            entities: HashMap::new(),
            relationships: HashMap::new(),
            schema_registry: SchemaRegistry::new(),
            active_schema: None,
        })
    }

    /// Get workspace name
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Set active schema
    #[wasm_bindgen]
    pub fn set_active_schema(&mut self, schema_id: String) -> Result<(), JsValue> {
        // Verify schema exists
        self.schema_registry.get(&schema_id)
            .map_err(to_js_error)?;

        self.active_schema = Some(schema_id);
        Ok(())
    }

    /// Get active schema ID
    #[wasm_bindgen]
    pub fn active_schema(&self) -> Option<String> {
        self.active_schema.clone()
    }

    /// Register a schema
    #[wasm_bindgen]
    pub fn register_schema(&self, schema: &WasmSchema) -> Result<(), JsValue> {
        self.schema_registry.register(schema.inner().clone())
            .map_err(to_js_error)
    }

    /// Load schema from YAML
    #[wasm_bindgen]
    pub fn load_schema_yaml(&self, yaml: String) -> Result<String, JsValue> {
        self.schema_registry.register_from_yaml(&yaml)
            .map_err(to_js_error)
    }

    /// Create a new entity
    #[wasm_bindgen]
    pub fn create_entity(&mut self, entity_type: String) -> WasmEntity {
        let entity = Entity::new(entity_type);
        let id = entity.id.to_string();

        let wasm_entity = WasmEntity::from_entity(entity.clone());
        self.entities.insert(id, entity);

        wasm_entity
    }

    /// Add an entity to the workspace
    #[wasm_bindgen]
    pub fn add_entity(&mut self, entity: &WasmEntity) -> Result<String, JsValue> {
        // Validate against active schema if set
        if let Some(schema_id) = &self.active_schema {
            let schema = self.schema_registry.get(schema_id)
                .map_err(to_js_error)?;
            let validator = rp_schema::Validator::new(&schema);
            validator.validate_entity(entity.inner())
                .map_err(to_js_error)?;
        }

        let id = entity.inner().id.to_string();
        self.entities.insert(id.clone(), entity.inner().clone());

        Ok(id)
    }

    /// Get an entity by ID
    #[wasm_bindgen]
    pub fn get_entity(&self, id: String) -> Result<WasmEntity, JsValue> {
        self.entities.get(&id)
            .map(|e| WasmEntity::from_entity(e.clone()))
            .ok_or_else(|| JsValue::from_str(&format!("Entity not found: {}", id)))
    }

    /// Get all entities
    #[wasm_bindgen]
    pub fn get_all_entities(&self) -> Vec<JsValue> {
        self.entities.values()
            .map(|e| {
                let wasm_entity = WasmEntity::from_entity(e.clone());
                wasm_entity.to_json().unwrap_or(JsValue::NULL)
            })
            .collect()
    }

    /// Get entities by type
    #[wasm_bindgen]
    pub fn get_entities_by_type(&self, entity_type: String) -> Vec<JsValue> {
        self.entities.values()
            .filter(|e| e.entity_type == entity_type)
            .map(|e| {
                let wasm_entity = WasmEntity::from_entity(e.clone());
                wasm_entity.to_json().unwrap_or(JsValue::NULL)
            })
            .collect()
    }

    /// Create a new relationship
    #[wasm_bindgen]
    pub fn create_relationship(&mut self, relationship_type: String) -> WasmRelationship {
        let rel = Relationship::new(relationship_type);
        let id = rel.id.to_string();

        let wasm_rel = WasmRelationship::from_relationship(rel.clone());
        self.relationships.insert(id, rel);

        wasm_rel
    }

    /// Add a relationship to the workspace
    #[wasm_bindgen]
    pub fn add_relationship(&mut self, relationship: &WasmRelationship) -> Result<String, JsValue> {
        // Validate against active schema if set
        if let Some(schema_id) = &self.active_schema {
            let schema = self.schema_registry.get(schema_id)
                .map_err(to_js_error)?;
            let validator = rp_schema::Validator::new(&schema);
            validator.validate_relationship(relationship.inner())
                .map_err(to_js_error)?;
        }

        let id = relationship.inner().id.to_string();
        self.relationships.insert(id.clone(), relationship.inner().clone());

        Ok(id)
    }

    /// Get entity count
    #[wasm_bindgen]
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Get relationship count
    #[wasm_bindgen]
    pub fn relationship_count(&self) -> usize {
        self.relationships.len()
    }

    /// Get statistics as JSON
    #[wasm_bindgen]
    pub fn stats(&self) -> JsValue {
        let stats = serde_json::json!({
            "name": self.name,
            "entity_count": self.entities.len(),
            "relationship_count": self.relationships.len(),
            "active_schema": self.active_schema,
        });

        serde_wasm_bindgen::to_value(&stats).unwrap_or(JsValue::NULL)
    }

    /// Clear all data
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.entities.clear();
        self.relationships.clear();
    }
}
