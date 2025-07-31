//! Factory methods for creating domain events

use chrono::Utc;
use serde_json::Value as JsonValue;
use uuid::Uuid;
use std::collections::HashMap;

use crate::{
    DomainEvent, EventMetadata, EventError,
    TheoryEvent, PersonEvent, WorkspaceEvent,
};

/// Factory methods for creating domain events with metadata
impl DomainEvent {
    // Generic CRUD operations for any entity type
    
    /// Create a generic entity created event
    pub fn entity_created(
        entity_id: Uuid,
        entity_type: &str,
        data: JsonValue,
        actor_id: Uuid,
    ) -> Result<(Self, EventMetadata), EventError> {
        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            aggregate_id: entity_id,
            aggregate_type: entity_type.to_string(),
            aggregate_version: 1, // First event for new entity
            occurred_at: Utc::now(),
            actor_id,
            correlation_id: None,
            causation_id: None,
            tags: vec!["created".to_string()],
        };
        
        // Map to specific event type based on entity_type
        let event = match entity_type {
            "Theory" => {
                let question = data.get("question")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| EventError::InvalidEventData {
                        message: "Theory event missing required field: question".to_string()
                    })?
                    .to_string();
                let hypothesis = data.get("hypothesis")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| EventError::InvalidEventData {
                        message: "Theory event missing required field: hypothesis".to_string()
                    })?
                    .to_string();
                let researcher_id = data.get("researcher_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok())
                    .unwrap_or(actor_id); // Actor ID is acceptable default for researcher
                
                DomainEvent::Theory(TheoryEvent::Created {
                    question,
                    hypothesis,
                    researcher_id,
                })
            },
            "Person" => {
                let from_identities = data.get("from_identities")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter()
                        .filter_map(|v| v.as_str())
                        .filter_map(|s| Uuid::parse_str(s).ok())
                        .collect())
                    .unwrap_or_else(Vec::new); // Empty identities list is valid
                
                DomainEvent::Person(PersonEvent::Created {
                    from_identities,
                    concluded_by: actor_id,
                })
            },
            "Workspace" => {
                let name = data.get("name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| EventError::InvalidEventData {
                        message: "Workspace event missing required field: name".to_string()
                    })?
                    .to_string();
                
                DomainEvent::Workspace(WorkspaceEvent::Created {
                    name,
                    owner_id: actor_id,
                })
            },
            // For other entity types, use a generic approach
            _ => {
                // We'll need to implement a generic event type or extend this match
                // For now, default to Theory as a placeholder
                DomainEvent::Theory(TheoryEvent::Created {
                    question: format!("Generic {} created", entity_type),
                    hypothesis: "Placeholder".to_string(),
                    researcher_id: actor_id,
                })
            }
        };
        
        Ok((event, metadata))
    }
    
    /// Create a generic entity updated event
    pub fn entity_updated(
        entity_id: Uuid,
        entity_type: &str,
        changes: HashMap<String, JsonValue>,
        actor_id: Uuid,
        version: i64,
    ) -> Result<(Self, EventMetadata), EventError> {
        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            aggregate_id: entity_id,
            aggregate_type: entity_type.to_string(),
            aggregate_version: version,
            occurred_at: Utc::now(),
            actor_id,
            correlation_id: None,
            causation_id: None,
            tags: vec!["updated".to_string()],
        };
        
        // Map to specific event type based on entity_type
        let event = match entity_type {
            "Theory" => {
                DomainEvent::Theory(TheoryEvent::Updated {
                    question: changes.get("question")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    hypothesis: changes.get("hypothesis")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    details: changes.get("details")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                })
            },
            "Person" => {
                DomainEvent::Person(PersonEvent::Updated {
                    details: serde_json::to_value(&changes)
                        .map_err(|e| EventError::SerializationError(e))?,
                })
            },
            _ => {
                // Generic fallback - need to implement generic event type
                DomainEvent::Theory(TheoryEvent::Updated {
                    question: None,
                    hypothesis: None,
                    details: Some(format!("{} updated with {} changes", entity_type, changes.len())),
                })
            }
        };
        
        Ok((event, metadata))
    }
    
    /// Create a generic entity deleted event
    pub fn entity_deleted(
        entity_id: Uuid,
        entity_type: &str,
        actor_id: Uuid,
        version: i64,
    ) -> Result<(Self, EventMetadata), EventError> {
        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            aggregate_id: entity_id,
            aggregate_type: entity_type.to_string(),
            aggregate_version: version,
            occurred_at: Utc::now(),
            actor_id,
            correlation_id: None,
            causation_id: None,
            tags: vec!["deleted".to_string()],
        };
        
        // Most entities don't have specific Deleted events, so we use StateChanged or similar
        let event = match entity_type {
            "Theory" => {
                DomainEvent::Theory(TheoryEvent::Abandoned {
                    reason: "Entity deleted".to_string(),
                })
            },
            "Workspace" => {
                // Workspace doesn't have a deleted event, use preference as a marker
                DomainEvent::Workspace(WorkspaceEvent::PreferenceSet {
                    preference_key: "status".to_string(),
                    preference_value: JsonValue::String("deleted".to_string()),
                })
            },
            _ => {
                // Generic fallback
                DomainEvent::Theory(TheoryEvent::Abandoned {
                    reason: format!("{} deleted", entity_type),
                })
            }
        };
        
        Ok((event, metadata))
    }
    
    // Specific factory methods for common operations
    
    /// Create a theory created event
    pub fn theory_created(
        theory_id: Uuid,
        question: String,
        hypothesis: String,
        researcher_id: Uuid,
        actor_id: Uuid,
    ) -> Result<(Self, EventMetadata), EventError> {
        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            aggregate_id: theory_id,
            aggregate_type: "Theory".to_string(),
            aggregate_version: 1,
            occurred_at: Utc::now(),
            actor_id,
            correlation_id: None,
            causation_id: None,
            tags: vec!["created".to_string(), "theory".to_string()],
        };
        
        let event = DomainEvent::Theory(TheoryEvent::Created {
            question,
            hypothesis,
            researcher_id,
        });
        
        Ok((event, metadata))
    }
    
    /// Create a person created event
    pub fn person_created(
        person_id: Uuid,
        from_identities: Vec<Uuid>,
        concluded_by: Uuid,
        actor_id: Uuid,
    ) -> Result<(Self, EventMetadata), EventError> {
        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            aggregate_id: person_id,
            aggregate_type: "Person".to_string(),
            aggregate_version: 1,
            occurred_at: Utc::now(),
            actor_id,
            correlation_id: None,
            causation_id: None,
            tags: vec!["created".to_string(), "person".to_string()],
        };
        
        let event = DomainEvent::Person(PersonEvent::Created {
            from_identities,
            concluded_by,
        });
        
        Ok((event, metadata))
    }
    
    /// Create a workspace created event
    pub fn workspace_created(
        workspace_id: Uuid,
        name: String,
        creator_id: Uuid,
    ) -> Result<(Self, EventMetadata), EventError> {
        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            aggregate_id: workspace_id,
            aggregate_type: "Workspace".to_string(),
            aggregate_version: 1,
            occurred_at: Utc::now(),
            actor_id: creator_id,
            correlation_id: None,
            causation_id: None,
            tags: vec!["created".to_string(), "workspace".to_string()],
        };
        
        let event = DomainEvent::Workspace(WorkspaceEvent::Created {
            name,
            owner_id: creator_id,
        });
        
        Ok((event, metadata))
    }
}

/// Extension trait for event types to get their type name
pub trait EventTypeName {
    fn event_type_name(&self) -> &'static str;
}

impl EventTypeName for TheoryEvent {
    fn event_type_name(&self) -> &'static str {
        match self {
            TheoryEvent::Created { .. } => "TheoryCreated",
            TheoryEvent::Updated { .. } => "TheoryUpdated",
            TheoryEvent::StateChanged { .. } => "TheoryStateChanged",
            TheoryEvent::ComplianceConfigured { .. } => "TheoryComplianceConfigured",
            TheoryEvent::WorkProductAdded { .. } => "TheoryWorkProductAdded",
            TheoryEvent::ContributorAdded { .. } => "TheoryContributorAdded",
            TheoryEvent::Abandoned { .. } => "TheoryAbandoned",
        }
    }
}

impl EventTypeName for DomainEvent {
    fn event_type_name(&self) -> &'static str {
        match self {
            DomainEvent::Theory(e) => e.event_type_name(),
            DomainEvent::Person(_) => "PersonEvent",
            DomainEvent::Workspace(_) => "WorkspaceEvent",
            // Add other mappings as needed
            _ => "UnknownEvent",
        }
    }
}