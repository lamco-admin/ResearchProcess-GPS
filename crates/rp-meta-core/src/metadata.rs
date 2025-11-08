//! Meta-information for tracking provenance and versioning

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{EntityId, Value};

/// Meta-information attached to all entities and relationships
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaInfo {
    /// When this was created
    pub created: DateTime<Utc>,

    /// When this was last modified
    pub modified: DateTime<Utc>,

    /// Who created this (entity ID of the creator)
    pub created_by: Option<EntityId>,

    /// Who last modified this (entity ID of the modifier)
    pub modified_by: Option<EntityId>,

    /// Version number (incremented on each modification)
    pub version: u64,

    /// Previous version ID (if this is a new version of an existing entity)
    pub previous_version: Option<EntityId>,

    /// Why this exists/was created
    pub rationale: Option<String>,

    /// Quality/review status
    pub review_status: Option<String>,

    /// Custom metadata fields
    pub custom: HashMap<String, Value>,
}

impl MetaInfo {
    /// Create new metadata for a newly created entity
    pub fn now() -> Self {
        let now = Utc::now();
        Self {
            created: now,
            modified: now,
            created_by: None,
            modified_by: None,
            version: 1,
            previous_version: None,
            rationale: None,
            review_status: None,
            custom: HashMap::new(),
        }
    }

    /// Create new metadata with a specific creator
    pub fn with_creator(created_by: EntityId) -> Self {
        let now = Utc::now();
        Self {
            created: now,
            modified: now,
            created_by: Some(created_by),
            modified_by: Some(created_by),
            version: 1,
            previous_version: None,
            rationale: None,
            review_status: None,
            custom: HashMap::new(),
        }
    }

    /// Update metadata for a modification
    pub fn update(&mut self, modified_by: Option<EntityId>) {
        self.modified = Utc::now();
        self.modified_by = modified_by;
    }

    /// Create metadata for a new version
    pub fn new_version(&self, previous_id: EntityId, modified_by: Option<EntityId>) -> Self {
        let now = Utc::now();
        Self {
            created: self.created,
            modified: now,
            created_by: self.created_by,
            modified_by,
            version: self.version + 1,
            previous_version: Some(previous_id),
            rationale: None,
            review_status: None,
            custom: self.custom.clone(),
        }
    }

    /// Set the rationale
    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = Some(rationale.into());
        self
    }

    /// Set the review status
    pub fn with_review_status(mut self, status: impl Into<String>) -> Self {
        self.review_status = Some(status.into());
        self
    }

    /// Add custom metadata
    pub fn with_custom(mut self, key: impl Into<String>, value: Value) -> Self {
        self.custom.insert(key.into(), value);
        self
    }

    /// Get custom metadata value
    pub fn get_custom(&self, key: &str) -> Option<&Value> {
        self.custom.get(key)
    }

    /// Check if this is the first version
    pub fn is_first_version(&self) -> bool {
        self.version == 1 && self.previous_version.is_none()
    }

    /// Check if this has been modified since creation
    pub fn is_modified(&self) -> bool {
        self.modified > self.created
    }
}

impl Default for MetaInfo {
    fn default() -> Self {
        Self::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_info_creation() {
        let meta = MetaInfo::now();
        assert_eq!(meta.version, 1);
        assert!(meta.is_first_version());
        assert!(!meta.is_modified());
    }

    #[test]
    fn test_meta_info_with_creator() {
        let creator = EntityId::new();
        let meta = MetaInfo::with_creator(creator);
        assert_eq!(meta.created_by, Some(creator));
        assert_eq!(meta.modified_by, Some(creator));
    }

    #[test]
    fn test_meta_info_update() {
        let mut meta = MetaInfo::now();
        let original_created = meta.created;

        std::thread::sleep(std::time::Duration::from_millis(10));
        let modifier = EntityId::new();
        meta.update(Some(modifier));

        assert_eq!(meta.created, original_created);
        assert!(meta.modified > meta.created);
        assert_eq!(meta.modified_by, Some(modifier));
        assert!(meta.is_modified());
    }

    #[test]
    fn test_meta_info_versioning() {
        let meta1 = MetaInfo::now();
        let id1 = EntityId::new();
        let modifier = EntityId::new();

        let meta2 = meta1.new_version(id1, Some(modifier));

        assert_eq!(meta2.version, 2);
        assert_eq!(meta2.previous_version, Some(id1));
        assert_eq!(meta2.created, meta1.created);
        assert!(meta2.modified > meta1.modified);
    }

    #[test]
    fn test_custom_metadata() {
        let meta = MetaInfo::now()
            .with_custom("key1", Value::Text("value1".to_string()))
            .with_custom("key2", Value::Integer(42));

        assert_eq!(
            meta.get_custom("key1"),
            Some(&Value::Text("value1".to_string()))
        );
        assert_eq!(meta.get_custom("key2"), Some(&Value::Integer(42)));
    }
}
