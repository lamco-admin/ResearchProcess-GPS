//! WASM bindings for Relationship

use wasm_bindgen::prelude::*;
use rp_meta_core::{Relationship, RelationshipId, Participant, EntityId};

use crate::utils::to_js_error;

/// WASM-friendly Relationship wrapper
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmRelationship {
    inner: Relationship,
}

#[wasm_bindgen]
impl WasmRelationship {
    /// Create a new relationship
    #[wasm_bindgen(constructor)]
    pub fn new(relationship_type: String) -> WasmRelationship {
        WasmRelationship {
            inner: Relationship::new(relationship_type),
        }
    }

    /// Get relationship ID as string
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.to_string()
    }

    /// Get relationship type
    #[wasm_bindgen(getter)]
    pub fn relationship_type(&self) -> String {
        self.inner.relationship_type.clone()
    }

    /// Add a participant
    #[wasm_bindgen]
    pub fn add_participant(&mut self, entity_id: String, role: String) -> Result<(), JsValue> {
        let id = EntityId::parse(&entity_id)
            .map_err(to_js_error)?;

        let participant = Participant::new(id, role);
        self.inner.add_participant(participant);

        Ok(())
    }

    /// Get participant count
    #[wasm_bindgen]
    pub fn participant_count(&self) -> usize {
        self.inner.participants.len()
    }

    /// Set a text property
    #[wasm_bindgen]
    pub fn set_text(&mut self, key: String, value: String) {
        self.inner.properties.set_text(key, value);
    }

    /// Convert to JSON
    #[wasm_bindgen]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(to_js_error)
    }

    /// Create from JSON
    #[wasm_bindgen]
    pub fn from_json(json: JsValue) -> Result<WasmRelationship, JsValue> {
        let inner: Relationship = serde_wasm_bindgen::from_value(json)
            .map_err(to_js_error)?;
        Ok(WasmRelationship { inner })
    }

    /// Validate the relationship
    #[wasm_bindgen]
    pub fn validate(&self) -> Result<(), JsValue> {
        self.inner.validate()
            .map_err(to_js_error)
    }
}

impl WasmRelationship {
    pub fn inner(&self) -> &Relationship {
        &self.inner
    }

    pub fn from_relationship(rel: Relationship) -> Self {
        WasmRelationship { inner: rel }
    }
}
