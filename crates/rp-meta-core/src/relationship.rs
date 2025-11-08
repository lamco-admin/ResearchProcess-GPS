//! Universal Relationship primitive - N-ary connections between entities

use serde::{Deserialize, Serialize};

use crate::{Certainty, Context, EntityId, MetaInfo, PropertyGraph, RelationshipId, Result};

/// Universal relationship connecting any number of entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relationship {
    /// Unique identifier
    pub id: RelationshipId,

    /// Relationship type (open-ended)
    /// Examples: "Kinship.Parent", "Identity.PossibleSame", "DNA.Match", "Located.At"
    pub relationship_type: String,

    /// Participants in this relationship (N-ary, not just binary!)
    pub participants: Vec<Participant>,

    /// Additional properties of the relationship
    pub properties: PropertyGraph,

    /// Contexts that qualify this relationship
    pub contexts: Vec<Context>,

    /// Meta-information
    pub meta: MetaInfo,
}

impl Relationship {
    /// Create a new relationship
    pub fn new(relationship_type: impl Into<String>) -> Self {
        Self {
            id: RelationshipId::new(),
            relationship_type: relationship_type.into(),
            participants: Vec::new(),
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
            meta: MetaInfo::now(),
        }
    }

    /// Create a new relationship with a specific ID
    pub fn with_id(id: RelationshipId, relationship_type: impl Into<String>) -> Self {
        Self {
            id,
            relationship_type: relationship_type.into(),
            participants: Vec::new(),
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
            meta: MetaInfo::now(),
        }
    }

    /// Create a relationship with a creator
    pub fn with_creator(relationship_type: impl Into<String>, created_by: EntityId) -> Self {
        Self {
            id: RelationshipId::new(),
            relationship_type: relationship_type.into(),
            participants: Vec::new(),
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
            meta: MetaInfo::with_creator(created_by),
        }
    }

    /// Add a participant
    pub fn with_participant(mut self, participant: Participant) -> Self {
        self.participants.push(participant);
        self
    }

    /// Add multiple participants
    pub fn with_participants(mut self, participants: Vec<Participant>) -> Self {
        self.participants.extend(participants);
        self
    }

    /// Add a property
    pub fn with_property(mut self, key: impl Into<String>, value: crate::Property) -> Self {
        self.properties.set(key, value);
        self
    }

    /// Add a context
    pub fn with_context(mut self, context: Context) -> Self {
        self.contexts.push(context);
        self
    }

    /// Add a participant
    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }

    /// Get participants by role
    pub fn get_participants_by_role(&self, role: &str) -> Vec<&Participant> {
        self.participants
            .iter()
            .filter(|p| p.role == role)
            .collect()
    }

    /// Get all entity IDs participating in this relationship
    pub fn participant_entity_ids(&self) -> Vec<EntityId> {
        self.participants.iter().map(|p| p.entity).collect()
    }

    /// Check if an entity participates in this relationship
    pub fn has_participant(&self, entity_id: EntityId) -> bool {
        self.participants.iter().any(|p| p.entity == entity_id)
    }

    /// Check if an entity participates with a specific role
    pub fn has_participant_with_role(&self, entity_id: EntityId, role: &str) -> bool {
        self.participants
            .iter()
            .any(|p| p.entity == entity_id && p.role == role)
    }

    /// Check if relationship has a specific context type
    pub fn has_context_type(&self, context_type: &str) -> bool {
        self.contexts.iter().any(|c| c.context_type == context_type)
    }

    /// Validate the relationship
    pub fn validate(&self) -> Result<()> {
        if self.relationship_type.is_empty() {
            return Err(crate::Error::validation_failed(
                "Relationship type cannot be empty",
            ));
        }
        if self.participants.is_empty() {
            return Err(crate::Error::validation_failed(
                "Relationship must have at least one participant",
            ));
        }
        Ok(())
    }

    /// Create a binary relationship (convenience method)
    pub fn binary(
        relationship_type: impl Into<String>,
        subject: EntityId,
        subject_role: impl Into<String>,
        object: EntityId,
        object_role: impl Into<String>,
    ) -> Self {
        Self::new(relationship_type)
            .with_participant(Participant::new(subject, subject_role))
            .with_participant(Participant::new(object, object_role))
    }
}

/// Participant in a relationship
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Participant {
    /// Entity participating in the relationship
    pub entity: EntityId,

    /// Role in the relationship (open-ended)
    /// Examples: "subject", "object", "parent", "child", "witness", "source"
    pub role: String,

    /// Certainty of this participation
    pub certainty: Certainty,

    /// Participant-specific contexts
    pub contexts: Vec<Context>,

    /// Participant-specific properties
    pub properties: PropertyGraph,
}

impl Participant {
    /// Create a new participant
    pub fn new(entity: EntityId, role: impl Into<String>) -> Self {
        Self {
            entity,
            role: role.into(),
            certainty: Certainty::unknown(),
            contexts: Vec::new(),
            properties: PropertyGraph::new(),
        }
    }

    /// Set certainty
    pub fn with_certainty(mut self, certainty: Certainty) -> Self {
        self.certainty = certainty;
        self
    }

    /// Add a context
    pub fn with_context(mut self, context: Context) -> Self {
        self.contexts.push(context);
        self
    }

    /// Add a property
    pub fn with_property(mut self, key: impl Into<String>, value: crate::Property) -> Self {
        self.properties.set(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Property, Value};

    #[test]
    fn test_relationship_creation() {
        let rel = Relationship::new("Kinship.Parent");
        assert_eq!(rel.relationship_type, "Kinship.Parent");
        assert!(rel.participants.is_empty());
    }

    #[test]
    fn test_binary_relationship() {
        let parent = EntityId::new();
        let child = EntityId::new();

        let rel = Relationship::binary("Kinship.Parent", parent, "parent", child, "child");

        assert_eq!(rel.participants.len(), 2);
        assert!(rel.has_participant(parent));
        assert!(rel.has_participant(child));
        assert!(rel.has_participant_with_role(parent, "parent"));
        assert!(rel.has_participant_with_role(child, "child"));
    }

    #[test]
    fn test_n_ary_relationship() {
        // DNA match involving person, sample, and lab (3-way relationship)
        let person = EntityId::new();
        let sample = EntityId::new();
        let lab = EntityId::new();

        let rel = Relationship::new("DNA.Match")
            .with_participant(Participant::new(person, "person"))
            .with_participant(Participant::new(sample, "sample"))
            .with_participant(Participant::new(lab, "laboratory"));

        assert_eq!(rel.participants.len(), 3);
        assert!(rel.has_participant_with_role(person, "person"));
        assert!(rel.has_participant_with_role(sample, "sample"));
        assert!(rel.has_participant_with_role(lab, "laboratory"));
    }

    #[test]
    fn test_relationship_with_properties() {
        let rel = Relationship::new("Kinship.Parent")
            .with_property("biological", Property::Value(Value::Boolean(true)))
            .with_property("adoptive", Property::Value(Value::Boolean(false)));

        assert!(rel.properties.get_bool("biological").unwrap());
        assert!(!rel.properties.get_bool("adoptive").unwrap());
    }

    #[test]
    fn test_relationship_with_context() {
        let ctx = Context::theoretical("Migration Theory", vec![]);
        let rel = Relationship::new("Kinship.Parent").with_context(ctx.clone());

        assert_eq!(rel.contexts.len(), 1);
        assert_eq!(rel.contexts[0].context_type, "Theoretical");
    }

    #[test]
    fn test_participant_with_certainty() {
        let entity = EntityId::new();
        let participant = Participant::new(entity, "subject")
            .with_certainty(Certainty::probability(0.75));

        match participant.certainty {
            Certainty::Probability(p) => assert_eq!(p, 0.75),
            _ => panic!("Expected Probability"),
        }
    }

    #[test]
    fn test_participant_with_context() {
        let entity = EntityId::new();
        let ctx = Context::temporal("1850s", None);
        let participant = Participant::new(entity, "subject").with_context(ctx);

        assert_eq!(participant.contexts.len(), 1);
    }

    #[test]
    fn test_get_participants_by_role() {
        let parent1 = EntityId::new();
        let parent2 = EntityId::new();
        let child = EntityId::new();

        let rel = Relationship::new("Family")
            .with_participant(Participant::new(parent1, "parent"))
            .with_participant(Participant::new(parent2, "parent"))
            .with_participant(Participant::new(child, "child"));

        let parents = rel.get_participants_by_role("parent");
        assert_eq!(parents.len(), 2);

        let children = rel.get_participants_by_role("child");
        assert_eq!(children.len(), 1);
    }

    #[test]
    fn test_participant_entity_ids() {
        let id1 = EntityId::new();
        let id2 = EntityId::new();
        let id3 = EntityId::new();

        let rel = Relationship::new("Group")
            .with_participant(Participant::new(id1, "member"))
            .with_participant(Participant::new(id2, "member"))
            .with_participant(Participant::new(id3, "leader"));

        let ids = rel.participant_entity_ids();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
        assert!(ids.contains(&id3));
    }

    #[test]
    fn test_relationship_validation() {
        let rel = Relationship::new("Test")
            .with_participant(Participant::new(EntityId::new(), "subject"));
        assert!(rel.validate().is_ok());

        let invalid_rel = Relationship::new("Test");
        assert!(invalid_rel.validate().is_err());

        let empty_type = Relationship {
            id: RelationshipId::new(),
            relationship_type: "".to_string(),
            participants: vec![Participant::new(EntityId::new(), "subject")],
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
            meta: MetaInfo::now(),
        };
        assert!(empty_type.validate().is_err());
    }

    #[test]
    fn test_identity_correlation_relationship() {
        // Example: Two personas that might be the same person
        let persona1 = EntityId::new();
        let persona2 = EntityId::new();

        let rel = Relationship::new("Identity.PossibleSame")
            .with_participant(
                Participant::new(persona1, "identity_a")
                    .with_certainty(Certainty::quantum(vec![
                        ("same_person", 0.75),
                        ("different_person", 0.25),
                    ])),
            )
            .with_participant(Participant::new(persona2, "identity_b"))
            .with_context(Context::evidential("Census Records", "Primary"));

        assert_eq!(rel.participants.len(), 2);
        assert!(rel.has_context_type("Evidential"));

        // Check quantum certainty on first participant
        match &rel.participants[0].certainty {
            Certainty::Quantum(states) => {
                assert_eq!(states.len(), 2);
                assert_eq!(states[0].0, "same_person");
            }
            _ => panic!("Expected Quantum certainty"),
        }
    }
}
