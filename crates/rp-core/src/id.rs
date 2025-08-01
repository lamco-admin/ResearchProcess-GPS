//! Entity ID implementation using UUID v7

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;
use utoipa::ToSchema;

/// Entity identifier using UUID v7 (time-ordered)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
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
        // UUID v7 has a 48-bit big-endian timestamp in milliseconds
        // The timestamp occupies bytes 0-5, with the version taking 4 bits of byte 6
        let bytes = self.0.as_bytes();
        
        // Extract the 48-bit timestamp
        let timestamp_ms = ((bytes[0] as u64) << 40)
            | ((bytes[1] as u64) << 32)
            | ((bytes[2] as u64) << 24)
            | ((bytes[3] as u64) << 16)
            | ((bytes[4] as u64) << 8)
            | (bytes[5] as u64);
        
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
        // Allow for small time differences or clock drift
        assert!(diff.num_seconds() < 60 && diff.num_seconds() >= -1);
    }
}