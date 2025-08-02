// Relationship - Connects entities with meaning

use super::{EntityId, RelationshipId, Context, Certainty, PropertyGraph};
use crate::common::MetaInfo;
use serde::{Serialize, Deserialize};

/// Relationship between entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Unique identifier
    pub id: RelationshipId,

    /// Open-ended relationship type
    /// Examples: "Parent.Child", "Identity.PossibleSame", "Evidence.Supports"
    pub relationship_type: String,

    /// Participants in this relationship
    pub participants: Vec<Participant>,

    /// Properties of the relationship itself
    pub properties: PropertyGraph,

    /// Contexts that scope this relationship
    pub contexts: Vec<Context>,

    /// Metadata
    pub meta: MetaInfo,
}

/// A participant in a relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// The entity participating
    pub entity: EntityId,

    /// Role in the relationship (open-ended)
    /// Examples: "parent", "child", "subject", "evidence"
    pub role: String,

    /// Certainty of this participation
    pub certainty: Certainty,

    /// Participant-specific contexts
    pub contexts: Vec<Context>,
}

impl Relationship {
    /// Create a new relationship
    pub fn new(relationship_type: impl Into<String>) -> Self {
        Relationship {
            id: RelationshipId::new(),
            relationship_type: relationship_type.into(),
            participants: Vec::new(),
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
            meta: MetaInfo::new(),
        }
    }

    /// Create with specific ID
    pub fn with_id(id: RelationshipId, relationship_type: impl Into<String>) -> Self {
        Relationship {
            id,
            relationship_type: relationship_type.into(),
            participants: Vec::new(),
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
            meta: MetaInfo::new(),
        }
    }

    /// Add a participant
    pub fn add_participant(&mut self, entity: EntityId, role: impl Into<String>, certainty: Certainty) {
        self.participants.push(Participant {
            entity,
            role: role.into(),
            certainty,
            contexts: Vec::new(),
        });
    }

    /// Add participant with contexts
    pub fn add_participant_with_contexts(
        &mut self,
        entity: EntityId,
        role: impl Into<String>,
        certainty: Certainty,
        contexts: Vec<Context>
    ) {
        self.participants.push(Participant {
            entity,
            role: role.into(),
            certainty,
            contexts,
        });
    }

    /// Get participants by role
    pub fn participants_by_role(&self, role: &str) -> Vec<&Participant> {
        self.participants
            .iter()
            .filter(|p| p.role == role)
            .collect()
    }

    /// Check if entity participates
    pub fn has_participant(&self, entity: EntityId) -> bool {
        self.participants.iter().any(|p| p.entity == entity)
    }

    /// Get all participating entities
    pub fn entities(&self) -> Vec<EntityId> {
        self.participants.iter().map(|p| p.entity).collect()
    }
}

/// Common relationship builders
impl Relationship {
    /// Create a parent-child relationship
    pub fn parent_child(parent: EntityId, child: EntityId, certainty: Certainty) -> Self {
        let mut rel = Relationship::new("Kinship.ParentChild");
        rel.add_participant(parent, "parent", certainty.clone());
        rel.add_participant(child, "child", certainty);
        rel
    }

    /// Create a possible-same identity relationship
    pub fn possible_same(entity_a: EntityId, entity_b: EntityId, probability: f64) -> Self {
        let mut rel = Relationship::new("Identity.PossibleSame");
        let certainty = Certainty::Quantum(vec![
            ("same_person".to_string(), probability),
            ("different_person".to_string(), 1.0 - probability),
        ]);
        rel.add_participant(entity_a, "identity_a", certainty.clone());
        rel.add_participant(entity_b, "identity_b", certainty);
        rel
    }

    /// Create an evidence relationship
    pub fn evidence_supports(evidence: EntityId, claim: EntityId, strength: f64) -> Self {
        let mut rel = Relationship::new("Evidence.Supports");
        rel.add_participant(evidence, "evidence", Certainty::Percentage(100.0));
        rel.add_participant(claim, "claim", Certainty::Percentage(strength * 100.0));
        rel.properties.set_text("strength", match strength {
            s if s >= 0.9 => "strong",
            s if s >= 0.7 => "moderate",
            s if s >= 0.5 => "weak",
            _ => "very_weak",
        });
        rel
    }
}