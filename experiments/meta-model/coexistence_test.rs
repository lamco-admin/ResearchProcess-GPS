// Demonstration: Multiple genealogical models coexisting in the same system

use crate::theoretical_meta_model::*;

/// Show how the same person can be represented in multiple models simultaneously
pub fn demonstrate_model_coexistence() {
    println!("=== Meta-Model Coexistence Demonstration ===\n");
    
    // The same historical person represented in different ways
    let historical_person = "John Smith, born ~1850";
    
    // 1. GEDCOM representation (conclusional)
    let gedcom_john = create_gedcom_person();
    
    // 2. ResearchProcess representation (theoretical)  
    let research_john = create_research_identity();
    
    // 3. Quantum representation (superposition)
    let quantum_john = create_quantum_identity();
    
    // 4. Graph representation
    let graph_john = create_graph_node();
    
    // Create relationships between these representations
    let same_historical_person = Relationship {
        id: RelationshipId::new(),
        relationship_type: "MetaModel.RepresentsSame".to_string(),
        participants: vec![
            Participant {
                entity: gedcom_john.id,
                role: "gedcom_view".to_string(),
                certainty: Certainty::Narrative("GEDCOM conclusional view".to_string()),
                contexts: vec![],
            },
            Participant {
                entity: research_john.id,
                role: "research_view".to_string(),
                certainty: Certainty::Narrative("Research process view".to_string()),
                contexts: vec![],
            },
            Participant {
                entity: quantum_john.id,
                role: "quantum_view".to_string(),
                certainty: Certainty::Narrative("Quantum superposition view".to_string()),
                contexts: vec![],
            },
            Participant {
                entity: graph_john.id,
                role: "graph_view".to_string(),
                certainty: Certainty::Narrative("Graph database view".to_string()),
                contexts: vec![],
            },
        ],
        properties: {
            let mut props = PropertyGraph::new();
            props.set_text("historical_person", historical_person);
            props.set_text("note", "Same person expressed in different models");
            props
        },
        contexts: vec![],
        meta: MetaInfo::now(),
    };
    
    println!("Created 4 different representations of {}", historical_person);
    println!("All connected by relationship: {}", same_historical_person.id.0);
}

fn create_gedcom_person() -> Entity {
    let mut person = Entity::new("GEDCOM.INDI");
    person.state = "Concluded";
    person.properties.set_text("name", "John /Smith/");
    person.properties.set_text("birth_date", "ABT 1850");
    person.properties.set_text("birth_place", "Boston, MA");
    println!("GEDCOM: Conclusional person with fixed facts");
    person
}

fn create_research_identity() -> Entity {
    let mut identity = Entity::new("Research.IdentityPersona");
    identity.state = "Hypothesis";
    identity.properties.set_text("primary_name", "John Smith");
    identity.properties.set("birth_theories", Property::Collection(vec![
        Property::Theoretical(TheoreticalValue {
            hypothesis: "Born 1850 based on 1870 census age".to_string(),
            supporting_evidence: vec![],
            contradicting_evidence: vec![],
            probability: 0.7,
        }),
        Property::Theoretical(TheoreticalValue {
            hypothesis: "Born 1848 based on death record".to_string(),
            supporting_evidence: vec![],
            contradicting_evidence: vec![],
            probability: 0.3,
        }),
    ]));
    println!("Research: Hypothesis with multiple theories");
    identity
}

fn create_quantum_identity() -> Entity {
    let mut quantum = Entity::new("Quantum.Identity");
    quantum.state = "Superposition";
    quantum.properties.set("collapsed_states", Property::Quantum(vec![
        (Property::Entity(Box::new({
            let mut state = Entity::new("Identity.State");
            state.properties.set_text("interpretation", "Single person John Smith");
            state.properties.set_text("birth", "1850 Boston");
            state
        })), 0.6),
        (Property::Entity(Box::new({
            let mut state = Entity::new("Identity.State");
            state.properties.set_text("interpretation", "Father and son both named John");
            state
        })), 0.3),
        (Property::Entity(Box::new({
            let mut state = Entity::new("Identity.State");
            state.properties.set_text("interpretation", "Transcription error - actually James");
            state
        })), 0.1),
    ]));
    println!("Quantum: Superposition of multiple possibilities");
    quantum
}

fn create_graph_node() -> Entity {
    let mut node = Entity::new("Graph.Node");
    node.state = "Active";
    node.properties.set_text("label", "Person");
    node.properties.set("attributes", Property::Map({
        let mut attrs = HashMap::new();
        attrs.insert("name".to_string(), Property::Value(Value::Text("John Smith".to_string())));
        attrs.insert("birth_year".to_string(), Property::Value(Value::Integer(1850)));
        attrs
    }));
    println!("Graph: Node with properties");
    node
}

/// Demonstrate identity correlation scenario
pub fn demonstrate_identity_correlation() {
    println!("\n=== Identity Correlation Demonstration ===\n");
    
    // Create "Bob Grandma's neighbor"
    let mut neighbor = Entity::new("Identity.Descriptive");
    neighbor.state = "Observed";
    neighbor.properties.set_text("description", "Bob Grandma's neighbor");
    neighbor.properties.set_text("source", "Bob Grandma's testimony, recorded 1995");
    neighbor.properties.set("characteristics", Property::Collection(vec![
        Property::Value(Value::Text("Lived next door in 1950s".to_string())),
        Property::Value(Value::Text("Had a dog named Spot".to_string())),
        Property::Value(Value::Text("Worked at the factory".to_string())),
    ]));
    
    // Create "Robert Jones" from census
    let mut robert = Entity::new("Identity.Documented");
    robert.state = "Recorded";
    robert.properties.set_text("name", "Robert Jones");
    robert.properties.set_text("source", "1950 Census, Ward 5, Household 142");
    robert.properties.set_text("occupation", "Factory worker");
    robert.properties.set_text("address", "125 Main St");
    
    // Create possible-same relationship
    let correlation = Relationship {
        id: RelationshipId::new(),
        relationship_type: "Identity.PossibleSame".to_string(),
        participants: vec![
            Participant {
                entity: neighbor.id,
                role: "descriptive_identity".to_string(),
                certainty: Certainty::Quantum(vec![
                    ("same_person".to_string(), 0.75),
                    ("different_person".to_string(), 0.25),
                ]),
                contexts: vec![],
            },
            Participant {
                entity: robert.id,
                role: "documented_identity".to_string(),
                certainty: Certainty::Quantum(vec![
                    ("same_person".to_string(), 0.75),
                    ("different_person".to_string(), 0.25),
                ]),
                contexts: vec![],
            },
        ],
        properties: {
            let mut props = PropertyGraph::new();
            props.set("correlation_points", Property::Collection(vec![
                Property::Value(Value::Text("Geography: 125 Main St is next to Bob Grandma".to_string())),
                Property::Value(Value::Text("Occupation: Factory worker matches".to_string())),
                Property::Value(Value::Text("Timeline: 1950 census during 1950s".to_string())),
            ]));
            props.set("research_needed", Property::Value(
                Value::Text("Check city directories for dog licenses (Spot)".to_string())
            ));
            props
        },
        contexts: vec![],
        meta: MetaInfo::now(),
    };
    
    println!("Created two identities:");
    println!("1. {} ({})", neighbor.id.0, neighbor.properties.get("description").unwrap());
    println!("2. {} ({})", robert.id.0, robert.properties.get("name").unwrap());
    println!("\nLinked by correlation: {}", correlation.id.0);
    println!("Probability same person: 75%");
    
    // Show queries
    println!("\n--- Query Examples ---");
    println!("Query: 'Show me Bob Grandma's neighbor'");
    println!("Result: Returns ONLY neighbor entity\n");
    
    println!("Query: 'Show me all possible identities for Bob Grandma's neighbor'");
    println!("Result: Returns neighbor AND Robert Jones with correlation\n");
    
    println!("Query: 'Show me Robert Jones'");
    println!("Result: Returns ONLY Robert Jones entity\n");
    
    // Show operations
    println!("--- Operation Examples ---");
    
    // Merge operation (theoretical)
    println!("\nOperation: Theoretical merge");
    let mut merged = Entity::new("Identity.Merged");
    merged.state = "Theoretical";
    merged.properties.set_text("primary_name", "Robert Jones");
    merged.properties.set_text("description", "Bob Grandma's neighbor (merged identity)");
    merged.properties.set("merge_sources", Property::Collection(vec![
        Property::Reference(neighbor.id),
        Property::Reference(robert.id),
    ]));
    merged.properties.set("merge_confidence", Property::Value(Value::Float(0.75)));
    println!("Created merged identity: {}", merged.id.0);
    
    // Split operation (reversal)
    println!("\nOperation: Split after new evidence");
    let split_evidence = Entity::new("Evidence.Documentary");
    split_evidence.properties.set_text("description", "Found Robert Jones at different address in 1951 directory");
    
    let split_relationship = Relationship {
        id: RelationshipId::new(),
        relationship_type: "Identity.ProvenDifferent".to_string(),
        participants: vec![
            Participant {
                entity: neighbor.id,
                role: "identity_a".to_string(),
                certainty: Certainty::Narrative("New evidence proves different".to_string()),
                contexts: vec![],
            },
            Participant {
                entity: robert.id,
                role: "identity_b".to_string(),
                certainty: Certainty::Narrative("New evidence proves different".to_string()),
                contexts: vec![],
            },
        ],
        properties: {
            let mut props = PropertyGraph::new();
            props.set_reference("evidence", split_evidence.id);
            props.set_reference("reverses_merge", merged.id);
            props
        },
        contexts: vec![],
        meta: MetaInfo::now(),
    };
    
    println!("Split relationship created: {}", split_relationship.id.0);
    println!("Identities now proven different");
}

/// Show how to query across model boundaries
pub fn demonstrate_cross_model_queries() {
    println!("\n=== Cross-Model Query Examples ===\n");
    
    println!("Query 1: Find all representations of John Smith");
    println!("- Traverse relationships where type = 'MetaModel.RepresentsSame'");
    println!("- Returns: GEDCOM.INDI, Research.Identity, Quantum.Identity, Graph.Node");
    
    println!("\nQuery 2: Get highest confidence birth date across all models");
    println!("- Check all entities in relationship");
    println!("- Extract birth data from each model's format");
    println!("- Weight by certainty/confidence");
    println!("- Result: 1850 (70% confidence from research model)");
    
    println!("\nQuery 3: Find all uncertain identities");
    println!("- Filter entities where:");
    println!("  - state IN ['Hypothesis', 'Superposition', 'Theoretical']");
    println!("  - OR has 'Identity.PossibleSame' relationships");
    println!("  - OR certainty < threshold");
    
    println!("\nQuery 4: Export to specific model");
    println!("- Given: Quantum superposition identity");
    println!("- Export to: GEDCOM");
    println!("- Process: Collapse quantum state, pick highest probability");
    println!("- Result: Single GEDCOM.INDI with notes about uncertainty");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_model_coexistence() {
        demonstrate_model_coexistence();
        demonstrate_identity_correlation();
        demonstrate_cross_model_queries();
    }
    
    #[test]
    fn test_identity_operations() {
        // Test that identities can be:
        // 1. Created separately
        // 2. Linked as possible-same
        // 3. Merged theoretically
        // 4. Split again
        // All while maintaining history
        
        let neighbor = Entity::new("Identity.Descriptive");
        let robert = Entity::new("Identity.Named");
        
        assert_ne!(neighbor.id, robert.id);
        assert_eq!(neighbor.relationships.len(), 0);
        assert_eq!(robert.relationships.len(), 0);
        
        // After linking, both should reference the relationship
        // (in real implementation)
    }
}