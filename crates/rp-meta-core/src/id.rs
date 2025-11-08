//! Unique identifiers for all meta-model entities

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

macro_rules! define_id_type {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            /// Create a new random ID
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            /// Create from an existing UUID
            pub fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            /// Get the underlying UUID
            pub fn as_uuid(&self) -> Uuid {
                self.0
            }

            /// Convert to bytes
            pub fn as_bytes(&self) -> &[u8; 16] {
                self.0.as_bytes()
            }

            /// Parse from string
            pub fn parse(s: &str) -> Result<Self, uuid::Error> {
                Ok(Self(Uuid::parse_str(s)?))
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<Uuid> for $name {
            fn from(uuid: Uuid) -> Self {
                Self(uuid)
            }
        }

        impl From<$name> for Uuid {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

// Layer 1: Core meta-model IDs
define_id_type!(EntityId, "Unique identifier for an Entity");
define_id_type!(RelationshipId, "Unique identifier for a Relationship");
define_id_type!(ContextId, "Unique identifier for a Context");
define_id_type!(PropertyId, "Unique identifier for a Property");

// Layer 2: Research process IDs
define_id_type!(ProcessId, "Unique identifier for a Process");
define_id_type!(ActivityId, "Unique identifier for an Activity");
define_id_type!(AgentId, "Unique identifier for an Agent");
define_id_type!(ProductId, "Unique identifier for a Product");
define_id_type!(MethodologyId, "Unique identifier for a Methodology");

// Layer 3: Workspace/configuration IDs
define_id_type!(WorkspaceId, "Unique identifier for a Workspace");
define_id_type!(ConfigurationId, "Unique identifier for a Configuration");
define_id_type!(ViewId, "Unique identifier for a View");
define_id_type!(ToolId, "Unique identifier for a Tool");
define_id_type!(NodeId, "Unique identifier for a Node");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_creation() {
        let id1 = EntityId::new();
        let id2 = EntityId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_id_roundtrip() {
        let id = EntityId::new();
        let uuid = id.as_uuid();
        let id2 = EntityId::from_uuid(uuid);
        assert_eq!(id, id2);
    }

    #[test]
    fn test_id_parse() {
        let id = EntityId::new();
        let s = id.to_string();
        let parsed = EntityId::parse(&s).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_id_serialization() {
        let id = EntityId::new();
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: EntityId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }
}
