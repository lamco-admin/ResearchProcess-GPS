//! WASM bindings for Schema

use wasm_bindgen::prelude::*;
use rp_schema::{Schema, SchemaRegistry, Validator};

use crate::utils::to_js_error;
use crate::entity::WasmEntity;

/// WASM-friendly Schema wrapper
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmSchema {
    inner: Schema,
}

#[wasm_bindgen]
impl WasmSchema {
    /// Load schema from YAML
    #[wasm_bindgen]
    pub fn from_yaml(yaml: String) -> Result<WasmSchema, JsValue> {
        let inner = Schema::from_yaml(&yaml)
            .map_err(to_js_error)?;
        Ok(WasmSchema { inner })
    }

    /// Load schema from JSON
    #[wasm_bindgen]
    pub fn from_json(json: String) -> Result<WasmSchema, JsValue> {
        let inner = Schema::from_json(&json)
            .map_err(to_js_error)?;
        Ok(WasmSchema { inner })
    }

    /// Get schema ID
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.clone()
    }

    /// Get schema name
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// Get schema version
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> String {
        self.inner.version.clone()
    }

    /// Convert to JSON
    #[wasm_bindgen]
    pub fn to_json(&self) -> Result<String, JsValue> {
        self.inner.to_json()
            .map_err(to_js_error)
    }

    /// Convert to YAML
    #[wasm_bindgen]
    pub fn to_yaml(&self) -> Result<String, JsValue> {
        self.inner.to_yaml()
            .map_err(to_js_error)
    }

    /// Validate an entity against this schema
    #[wasm_bindgen]
    pub fn validate_entity(&self, entity: &WasmEntity) -> Result<(), JsValue> {
        let validator = Validator::new(&self.inner);
        validator.validate_entity(entity.inner())
            .map(|_| ())
            .map_err(to_js_error)
    }

    /// List all entity types in this schema
    #[wasm_bindgen]
    pub fn entity_types(&self) -> Vec<JsValue> {
        self.inner.entity_types.keys()
            .map(|k| JsValue::from_str(k))
            .collect()
    }

    /// List all relationship types in this schema
    #[wasm_bindgen]
    pub fn relationship_types(&self) -> Vec<JsValue> {
        self.inner.relationship_types.keys()
            .map(|k| JsValue::from_str(k))
            .collect()
    }
}

impl WasmSchema {
    pub fn inner(&self) -> &Schema {
        &self.inner
    }
}

/// WASM-friendly Schema Registry
#[wasm_bindgen]
pub struct WasmSchemaRegistry {
    inner: SchemaRegistry,
}

#[wasm_bindgen]
impl WasmSchemaRegistry {
    /// Create a new registry
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmSchemaRegistry {
        WasmSchemaRegistry {
            inner: SchemaRegistry::new(),
        }
    }

    /// Register a schema
    #[wasm_bindgen]
    pub fn register(&self, schema: &WasmSchema) -> Result<(), JsValue> {
        self.inner.register(schema.inner.clone())
            .map_err(to_js_error)
    }

    /// Get a schema by ID
    #[wasm_bindgen]
    pub fn get(&self, schema_id: String) -> Result<WasmSchema, JsValue> {
        let inner = self.inner.get(&schema_id)
            .map_err(to_js_error)?;
        Ok(WasmSchema { inner })
    }

    /// List all schema IDs
    #[wasm_bindgen]
    pub fn list(&self) -> Result<Vec<JsValue>, JsValue> {
        self.inner.list_schemas()
            .map(|ids| ids.into_iter().map(|id| JsValue::from_str(&id)).collect())
            .map_err(to_js_error)
    }
}
