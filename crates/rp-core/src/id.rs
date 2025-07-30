//! Entity ID implementation using UUID v7

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Entity identifier using UUID v7 (time-ordered)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EntityId(Uuid);

impl EntityId {
    /// Generate a new entity ID
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
    
    /// Create from an existing UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    /// Get the underlying UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
    
    /// Convert to a hyphenated string
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    
    /// Parse from a string
    pub fn parse_str(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
    
    /// Get the timestamp from this UUID v7
    pub fn timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        // UUID v7 has timestamp in the first 48 bits
        let bytes = self.0.as_bytes();
        let timestamp_ms = u64::from_be_bytes([
            0, 0,
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
        ]) >> 4; // Remove version bits
        
        chrono::DateTime::from_timestamp_millis(timestamp_ms as i64)
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for EntityId {
    type Err = uuid::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl From<Uuid> for EntityId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<EntityId> for Uuid {
    fn from(id: EntityId) -> Self {
        id.0
    }
}

/// Generate a new entity ID
pub fn generate_id() -> EntityId {
    EntityId::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_id_creation() {
        let id1 = EntityId::new();
        let id2 = EntityId::new();
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_entity_id_string_conversion() {
        let id = EntityId::new();
        let s = id.to_string();
        let parsed = EntityId::parse_str(&s).unwrap();
        assert_eq!(id, parsed);
    }
    
    #[test]
    fn test_entity_id_timestamp() {
        let id = EntityId::new();
        let timestamp = id.timestamp();
        assert!(timestamp.is_some());
        
        // Check that timestamp is recent (within last minute)
        let now = chrono::Utc::now();
        let ts = timestamp.unwrap();
        let diff = now - ts;
        assert!(diff.num_seconds() < 60);
    }
}