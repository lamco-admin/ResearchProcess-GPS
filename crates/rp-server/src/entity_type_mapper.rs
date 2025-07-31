use rp_core::layer3::EntityType;

/// Parse entity type from string representation
pub fn parse_entity_type(type_str: &str) -> EntityType {
    match type_str {
        "Theory" => EntityType::Theory,
        "Evidence" => EntityType::Evidence,
        "Source" => EntityType::Source,
        "Repository" => EntityType::Repository,
        "WorkProduct" => EntityType::WorkProduct,
        "ProofStatement" => EntityType::ProofStatement,
        "Researcher" => EntityType::Researcher,
        "ResearchLog" => EntityType::ResearchLog,
        "Citation" => EntityType::Citation,
        "Fact" => EntityType::Fact,
        "IdentityPersona" => EntityType::IdentityPersona,
        "Relationship" => EntityType::Relationship,
        // Map common entity names to available types
        "Person" => EntityType::IdentityPersona,
        "Document" => EntityType::WorkProduct,
        "Analysis" => EntityType::Theory,
        // Default to Theory for unknown types
        _ => {
            tracing::warn!("Unknown entity type '{}', defaulting to Theory", type_str);
            EntityType::Theory
        }
    }
}

/// Get string representation of entity type
pub fn entity_type_to_string(entity_type: &EntityType) -> &'static str {
    match entity_type {
        EntityType::Theory => "Theory",
        EntityType::Evidence => "Evidence",
        EntityType::Source => "Source",
        EntityType::Repository => "Repository",
        EntityType::WorkProduct => "WorkProduct",
        EntityType::ProofStatement => "ProofStatement",
        EntityType::Researcher => "Researcher",
        EntityType::ResearchLog => "ResearchLog",
        EntityType::Citation => "Citation",
        EntityType::Fact => "Fact",
        EntityType::IdentityPersona => "IdentityPersona",
        EntityType::Relationship => "Relationship",
    }
}