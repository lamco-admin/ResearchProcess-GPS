//! WASM bindings for Entity

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use rp_meta_core::{Entity, EntityId, Property, PropertyGraph, Value};

use crate::utils::to_js_error;

/// WASM-friendly Entity wrapper
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmEntity {
    inner: Entity,
}

#[wasm_bindgen]
impl WasmEntity {
    /// Create a new entity
    #[wasm_bindgen(constructor)]
    pub fn new(entity_type: String) -> WasmEntity {
        WasmEntity {
            inner: Entity::new(entity_type),
        }
    }

    /// Get entity ID as string
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.to_string()
    }

    /// Get entity type
    #[wasm_bindgen(getter)]
    pub fn entity_type(&self) -> String {
        self.inner.entity_type.clone()
    }

    /// Get entity state
    #[wasm_bindgen(getter)]
    pub fn state(&self) -> String {
        self.inner.state.clone()
    }

    /// Set entity state
    #[wasm_bindgen(setter)]
    pub fn set_state(&mut self, state: String) {
        self.inner.set_state(state);
    }

    /// Set a text property
    #[wasm_bindgen]
    pub fn set_text(&mut self, key: String, value: String) {
        self.inner.properties.set_text(key, value);
    }

    /// Get a text property
    #[wasm_bindgen]
    pub fn get_text(&self, key: &str) -> Result<String, JsValue> {
        self.inner.properties.get_text(key)
            .map(|s| s.to_string())
            .map_err(to_js_error)
    }

    /// Set an integer property
    #[wasm_bindgen]
    pub fn set_integer(&mut self, key: String, value: i64) {
        self.inner.properties.set_integer(key, value);
    }

    /// Get an integer property
    #[wasm_bindgen]
    pub fn get_integer(&self, key: &str) -> Result<i64, JsValue> {
        self.inner.properties.get_integer(key)
            .map_err(to_js_error)
    }

    /// Set a boolean property
    #[wasm_bindgen]
    pub fn set_bool(&mut self, key: String, value: bool) {
        self.inner.properties.set_bool(key, value);
    }

    /// Get a boolean property
    #[wasm_bindgen]
    pub fn get_bool(&self, key: &str) -> Result<bool, JsValue> {
        self.inner.properties.get_bool(key)
            .map_err(to_js_error)
    }

    /// Convert to JSON
    #[wasm_bindgen]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(to_js_error)
    }

    /// Create from JSON
    #[wasm_bindgen]
    pub fn from_json(json: JsValue) -> Result<WasmEntity, JsValue> {
        let inner: Entity = serde_wasm_bindgen::from_value(json)
            .map_err(to_js_error)?;
        Ok(WasmEntity { inner })
    }

    /// Validate the entity
    #[wasm_bindgen]
    pub fn validate(&self) -> Result<(), JsValue> {
        self.inner.validate()
            .map_err(to_js_error)
    }
}

impl WasmEntity {
    /// Get the inner Entity
    pub fn inner(&self) -> &Entity {
        &self.inner
    }

    /// Create from Entity
    pub fn from_entity(entity: Entity) -> Self {
        WasmEntity { inner: entity }
    }
}
