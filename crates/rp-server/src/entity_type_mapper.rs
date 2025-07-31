use rp_core::layer3::EntityType;
use crate::error::ApiError;

/// Parse entity type from string representation
pub fn parse_entity_type(type_str: &str) -> Result<EntityType, ApiError> {
    match type_str {
        // Layer 1 - Core Data Model
        "Analysis" => Ok(EntityType::Analysis),
        "Citation" => Ok(EntityType::Citation),
        "Confidence" => Ok(EntityType::Confidence),
        "Evidence" => Ok(EntityType::Evidence),
        "Fact" => Ok(EntityType::Fact),
        "IdentityPersona" => Ok(EntityType::IdentityPersona),
        "Location" => Ok(EntityType::Location),
        "Relationship" => Ok(EntityType::Relationship),
        "Source" => Ok(EntityType::Source),
        
        // Layer 2 - Research Process & Products
        "AnalysisReport" => Ok(EntityType::AnalysisReport),
        "ProofStatement" => Ok(EntityType::ProofStatement),
        "ResearchActivity" => Ok(EntityType::ResearchActivity),
        "Researcher" => Ok(EntityType::Researcher),
        "ResearchLog" => Ok(EntityType::ResearchLog),
        "ResearchSession" => Ok(EntityType::ResearchSession),
        "Theory" => Ok(EntityType::Theory),
        "WorkProduct" => Ok(EntityType::WorkProduct),
        
        // Layer 3 - Workflow & Configuration
        "Workspace" => Ok(EntityType::Workspace),
        
        // Aliases for backward compatibility
        "Person" => Ok(EntityType::IdentityPersona),
        "Document" => Ok(EntityType::WorkProduct),
        "EvidenceAnalysis" => Ok(EntityType::AnalysisReport), // Old name compatibility
        
        // NO FALLBACK - Return error for unknown types (NO_FALLBACK_POLICY)
        _ => Err(ApiError::Validation(format!("Unknown entity type: '{}'", type_str)))
    }
}

/// Get string representation of entity type
pub fn entity_type_to_string(entity_type: &EntityType) -> &'static str {
    match entity_type {
        // Layer 1 - Core Data Model
        EntityType::Analysis => "Analysis",
        EntityType::Citation => "Citation",
        EntityType::Confidence => "Confidence",
        EntityType::Evidence => "Evidence",
        EntityType::Fact => "Fact",
        EntityType::IdentityPersona => "IdentityPersona",
        EntityType::Location => "Location",
        EntityType::Relationship => "Relationship",
        EntityType::Source => "Source",
        
        // Layer 2 - Research Process & Products
        EntityType::AnalysisReport => "AnalysisReport",
        EntityType::ProofStatement => "ProofStatement",
        EntityType::ResearchActivity => "ResearchActivity",
        EntityType::Researcher => "Researcher",
        EntityType::ResearchLog => "ResearchLog",
        EntityType::ResearchSession => "ResearchSession",
        EntityType::Theory => "Theory",
        EntityType::WorkProduct => "WorkProduct",
        
        // Layer 3 - Workflow & Configuration
        EntityType::Workspace => "Workspace",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_entity_types() {
        // Test all valid entity types
        assert!(parse_entity_type("Analysis").is_ok());
        assert!(parse_entity_type("Citation").is_ok());
        assert!(parse_entity_type("Confidence").is_ok());
        assert!(parse_entity_type("Evidence").is_ok());
        assert!(parse_entity_type("Fact").is_ok());
        assert!(parse_entity_type("IdentityPersona").is_ok());
        assert!(parse_entity_type("Location").is_ok());
        assert!(parse_entity_type("Relationship").is_ok());
        assert!(parse_entity_type("Source").is_ok());
        assert!(parse_entity_type("AnalysisReport").is_ok());
        assert!(parse_entity_type("ProofStatement").is_ok());
        assert!(parse_entity_type("ResearchActivity").is_ok());
        assert!(parse_entity_type("Researcher").is_ok());
        assert!(parse_entity_type("ResearchLog").is_ok());
        assert!(parse_entity_type("ResearchSession").is_ok());
        assert!(parse_entity_type("Theory").is_ok());
        assert!(parse_entity_type("WorkProduct").is_ok());
        assert!(parse_entity_type("Workspace").is_ok());
    }

    #[test]
    fn test_parse_aliases() {
        // Test backward compatibility aliases
        assert_eq!(parse_entity_type("Person").unwrap(), EntityType::IdentityPersona);
        assert_eq!(parse_entity_type("Document").unwrap(), EntityType::WorkProduct);
        assert_eq!(parse_entity_type("EvidenceAnalysis").unwrap(), EntityType::AnalysisReport);
    }

    #[test]
    fn test_parse_unknown_type_returns_error() {
        // Test NO_FALLBACK_POLICY - unknown types must return errors
        assert!(parse_entity_type("Unknown").is_err());
        assert!(parse_entity_type("Repository").is_err()); // No longer a valid entity type
        assert!(parse_entity_type("").is_err());
        assert!(parse_entity_type("SomeRandomType").is_err());
    }

    #[test]
    fn test_entity_type_to_string() {
        // Test string conversion for all types
        assert_eq!(entity_type_to_string(&EntityType::Analysis), "Analysis");
        assert_eq!(entity_type_to_string(&EntityType::Theory), "Theory");
        assert_eq!(entity_type_to_string(&EntityType::Workspace), "Workspace");
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test that we can convert to string and back for all types
        let all_types = vec![
            EntityType::Analysis,
            EntityType::Citation,
            EntityType::Confidence,
            EntityType::Evidence,
            EntityType::Fact,
            EntityType::IdentityPersona,
            EntityType::Location,
            EntityType::Relationship,
            EntityType::Source,
            EntityType::AnalysisReport,
            EntityType::ProofStatement,
            EntityType::ResearchActivity,
            EntityType::Researcher,
            EntityType::ResearchLog,
            EntityType::ResearchSession,
            EntityType::Theory,
            EntityType::WorkProduct,
            EntityType::Workspace,
        ];

        for entity_type in all_types {
            let string_repr = entity_type_to_string(&entity_type);
            let parsed = parse_entity_type(string_repr).unwrap();
            assert_eq!(parsed, entity_type);
        }
    }
}