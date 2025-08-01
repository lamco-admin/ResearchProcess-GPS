// Examples showing how the meta-model expresses different genealogical data models

use crate::theoretical_meta_model::*;

/// Example 1: Express traditional GEDCOM-style data
pub mod gedcom_expression {
    use super::*;
    
    pub fn express_gedcom_person() -> Entity {
        let mut person = Entity::new("GEDCOM.INDI");
        person.state = "Concluded"; // GEDCOM assumes conclusions
        
        // Name as nested entity
        let mut name = Entity::new("GEDCOM.NAME");
        name.properties.set_text("given", "John");
        name.properties.set_text("surname", "Smith");
        name.properties.set_text("type", "birth");
        person.properties.set_entity("name", name);
        
        // Birth event as reference
        let birth_event = EntityId::new(); // Would be actual birth event
        person.properties.set_reference("birth", birth_event);
        
        person
    }
    
    pub fn express_gedcom_family() -> Relationship {
        Relationship {
            id: RelationshipId::new(),
            relationship_type: "GEDCOM.FAM".to_string(),
            participants: vec![
                Participant {
                    entity: EntityId::new(), // Father
                    role: "HUSB".to_string(),
                    certainty: Certainty::Narrative("Assumed certain in GEDCOM".to_string()),
                    contexts: vec![],
                },
                Participant {
                    entity: EntityId::new(), // Mother
                    role: "WIFE".to_string(),
                    certainty: Certainty::Narrative("Assumed certain in GEDCOM".to_string()),
                    contexts: vec![],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
            meta: MetaInfo::now(),
        }
    }
}

/// Example 2: Express GEDCOM X style
pub mod gedcomx_expression {
    use super::*;
    
    pub fn express_gedcomx_person() -> Entity {
        let mut person = Entity::new("GedcomX.Person");
        person.state = "Documented";
        
        // GEDCOM X style facts as nested entities
        let mut birth_fact = Entity::new("GedcomX.Fact");
        birth_fact.properties.set_text("type", "Birth");
        birth_fact.properties.set("date", Property::Value(Value::Temporal(
            TemporalValue::Instant(TemporalInstant {
                expressions: vec![CalendarExpression::Gregorian { 
                    year: 1850, 
                    month: Some(1), 
                    day: Some(15) 
                }],
                precision: TemporalPrecision::Day,
                quality: TemporalQuality::Exact,
            })
        )));
        
        person.properties.set("facts", Property::Collection(vec![
            Property::Entity(Box::new(birth_fact))
        ]));
        
        person
    }
}

/// Example 3: Express ResearchProcess-GPS style
pub mod research_process_expression {
    use super::*;
    
    pub fn express_identity_hypothesis() -> Entity {
        let mut identity = Entity::new("ResearchProcess.IdentityPersona");
        identity.state = "Hypothesis"; // Key difference - not concluded!
        
        identity.properties.set_text("primary_name", "John Smith");
        identity.properties.set("alternative_names", Property::Collection(vec![
            Property::Value(Value::Text("J. Smith".to_string())),
            Property::Value(Value::Text("Johnny Smith".to_string())),
        ]));
        
        // Evidence references, not facts
        identity.properties.set("evidence_refs", Property::Collection(vec![
            Property::Reference(EntityId::new()),
            Property::Reference(EntityId::new()),
        ]));
        
        // Research contexts
        identity.contexts.push(Context {
            id: ContextId::new(),
            context_type: "Research.Status".to_string(),
            scope: Scope::Theoretical(TheoreticalScope {
                theory: "Working hypothesis about John Smith identity".to_string(),
                assumptions: vec![
                    "Census record refers to same person".to_string(),
                    "Age progression is consistent".to_string(),
                ],
            }),
            certainty: Certainty::Fuzzy { membership: 0.7, confidence: 0.8 },
            properties: PropertyGraph::new(),
        });
        
        identity
    }
    
    pub fn express_floating_evidence() -> Entity {
        let mut evidence = Entity::new("ResearchProcess.Evidence");
        evidence.state = "Unassigned"; // Not locked to any identity
        
        evidence.properties.set_text("description", "1850 Census entry for J. Smith");
        
        // Multiple possible interpretations
        evidence.properties.set("interpretations", Property::Quantum(vec![
            (Property::Reference(EntityId::new()), 0.6), // 60% likely John Smith Sr.
            (Property::Reference(EntityId::new()), 0.3), // 30% likely John Smith Jr.
            (Property::Value(Value::Text("Unknown Smith".to_string())), 0.1), // 10% someone else
        ]));
        
        evidence
    }
}

/// Example 4: Express "Bob Grandma's neighbor" scenario
pub mod identity_correlation_expression {
    use super::*;
    
    pub fn create_descriptive_identity() -> Entity {
        let mut neighbor = Entity::new("Identity.Descriptive");
        neighbor.state = "Observed";
        
        neighbor.properties.set_text("description", "Bob Grandma's neighbor");
        neighbor.properties.set_text("known_through", "Bob Grandma's testimony");
        
        // Temporal context
        neighbor.contexts.push(Context {
            id: ContextId::new(),
            context_type: "Temporal".to_string(),
            scope: Scope::Temporal(TemporalScope {
                description: "1950s".to_string(),
                bounds: Some((
                    TemporalValue::Instant(TemporalInstant {
                        expressions: vec![CalendarExpression::Gregorian { 
                            year: 1950, month: None, day: None 
                        }],
                        precision: TemporalPrecision::Year,
                        quality: TemporalQuality::Approximate,
                    }),
                    TemporalValue::Instant(TemporalInstant {
                        expressions: vec![CalendarExpression::Gregorian { 
                            year: 1959, month: None, day: None 
                        }],
                        precision: TemporalPrecision::Year,
                        quality: TemporalQuality::Approximate,
                    }),
                )),
            }),
            certainty: Certainty::Narrative("Based on Bob Grandma's recollection".to_string()),
            properties: PropertyGraph::new(),
        });
        
        neighbor
    }
    
    pub fn create_named_identity() -> Entity {
        let mut robert = Entity::new("Identity.Named");
        robert.state = "Documented";
        
        robert.properties.set_text("name", "Robert Jones");
        robert.properties.set_reference("source", EntityId::new()); // 1950 Census
        
        robert
    }
    
    pub fn create_possible_same_relationship(neighbor_id: EntityId, robert_id: EntityId) -> Relationship {
        Relationship {
            id: RelationshipId::new(),
            relationship_type: "Identity.PossibleSame".to_string(),
            participants: vec![
                Participant {
                    entity: neighbor_id,
                    role: "identity_a".to_string(),
                    certainty: Certainty::Quantum(vec![
                        ("is_same_person".to_string(), 0.75),
                        ("is_different_person".to_string(), 0.25),
                    ]),
                    contexts: vec![],
                },
                Participant {
                    entity: robert_id,
                    role: "identity_b".to_string(),
                    certainty: Certainty::Quantum(vec![
                        ("is_same_person".to_string(), 0.75),
                        ("is_different_person".to_string(), 0.25),
                    ]),
                    contexts: vec![],
                },
            ],
            properties: {
                let mut props = PropertyGraph::new();
                props.set("evidence_for", Property::Collection(vec![
                    Property::Value(Value::Text("Lived next door in 1950s".to_string())),
                    Property::Value(Value::Text("Age matches testimony".to_string())),
                ]));
                props.set("evidence_against", Property::Collection(vec![
                    Property::Value(Value::Text("No direct statement they're same".to_string())),
                ]));
                props
            },
            contexts: vec![],
            meta: MetaInfo::now(),
        }
    }
}

/// Example 5: Express FamilySearch FamilyTree style
pub mod familysearch_expression {
    use super::*;
    
    pub fn express_living_person() -> Entity {
        let mut person = Entity::new("FamilySearch.Person");
        person.state = "Living";
        
        // FamilySearch's conclusion model with contributors
        let mut conclusion = Entity::new("FamilySearch.Conclusion");
        conclusion.properties.set_reference("contributor", EntityId::new());
        conclusion.properties.set_text("confidence", "High");
        
        person.properties.set_entity("preferred_name", conclusion);
        
        // Change history as contexts
        person.contexts.push(Context {
            id: ContextId::new(),
            context_type: "ChangeHistory".to_string(),
            scope: Scope::Temporal(TemporalScope {
                description: "Last modified".to_string(),
                bounds: None,
            }),
            certainty: Certainty::Narrative("System tracked".to_string()),
            properties: {
                let mut props = PropertyGraph::new();
                props.set_reference("changed_by", EntityId::new());
                props.set_text("change_reason", "Corrected birth date");
                props
            },
        });
        
        person
    }
}

/// Example 6: Express graph database style
pub mod graph_expression {
    use super::*;
    
    pub fn express_as_graph_node() -> Entity {
        let mut node = Entity::new("Graph.Node");
        node.state = "Active";
        
        // Everything is properties in graph model
        node.properties.set_text("label", "Person");
        node.properties.set_text("name", "John Smith");
        node.properties.set("birth_year", Property::Value(Value::Integer(1850)));
        
        // Edges are just relationships
        node.relationships.push(RelationshipId::new());
        
        node
    }
    
    pub fn express_as_graph_edge(from: EntityId, to: EntityId) -> Relationship {
        Relationship {
            id: RelationshipId::new(),
            relationship_type: "Graph.Edge".to_string(),
            participants: vec![
                Participant {
                    entity: from,
                    role: "from".to_string(),
                    certainty: Certainty::Unknown,
                    contexts: vec![],
                },
                Participant {
                    entity: to,
                    role: "to".to_string(),
                    certainty: Certainty::Unknown,
                    contexts: vec![],
                },
            ],
            properties: {
                let mut props = PropertyGraph::new();
                props.set_text("label", "PARENT_OF");
                props
            },
            contexts: vec![],
            meta: MetaInfo::now(),
        }
    }
}

/// Example 7: Express theoretical quantum genealogy
pub mod quantum_expression {
    use super::*;
    
    pub fn express_superposition_identity() -> Entity {
        let mut quantum_person = Entity::new("Quantum.Identity");
        quantum_person.state = "Superposition";
        
        // Person exists in multiple states simultaneously
        quantum_person.properties.set("states", Property::Quantum(vec![
            (Property::Entity(Box::new({
                let mut state1 = Entity::new("Identity.State");
                state1.properties.set_text("name", "John Smith of Boston");
                state1.properties.set_text("birth", "1850");
                state1
            })), 0.4),
            (Property::Entity(Box::new({
                let mut state2 = Entity::new("Identity.State");
                state2.properties.set_text("name", "John Smith of New York");  
                state2.properties.set_text("birth", "1852");
                state2
            })), 0.35),
            (Property::Entity(Box::new({
                let mut state3 = Entity::new("Identity.State");
                state3.properties.set_text("name", "Two different people");
                state3
            })), 0.25),
        ]));
        
        quantum_person
    }
}

/// Example 8: Express DNA-based relationships
pub mod dna_expression {
    use super::*;
    
    pub fn express_genetic_relationship(person_a: EntityId, person_b: EntityId) -> Relationship {
        Relationship {
            id: RelationshipId::new(),
            relationship_type: "Genetic.Match".to_string(),
            participants: vec![
                Participant {
                    entity: person_a,
                    role: "match_a".to_string(),
                    certainty: Certainty::Narrative("DNA test confirmed".to_string()),
                    contexts: vec![],
                },
                Participant {
                    entity: person_b,
                    role: "match_b".to_string(),
                    certainty: Certainty::Narrative("DNA test confirmed".to_string()),
                    contexts: vec![],
                },
            ],
            properties: {
                let mut props = PropertyGraph::new();
                props.set("shared_cm", Property::Value(Value::Quantity(1750.0, "cM".to_string())));
                props.set("relationship_predictions", Property::Collection(vec![
                    Property::Theoretical(TheoreticalValue {
                        hypothesis: "First cousins".to_string(),
                        supporting_evidence: vec![],
                        contradicting_evidence: vec![],
                        probability: 0.65,
                    }),
                    Property::Theoretical(TheoreticalValue {
                        hypothesis: "Half siblings".to_string(),
                        supporting_evidence: vec![],
                        contradicting_evidence: vec![],
                        probability: 0.30,
                    }),
                ]));
                props
            },
            contexts: vec![],
            meta: MetaInfo::now(),
        }
    }
}

/// Demonstrate queries across models
pub mod query_examples {
    use super::*;
    
    // Find all entities that might be the same person
    pub fn find_possible_same_identities(target: EntityId) -> Vec<EntityId> {
        // In real implementation, would query relationships where:
        // - relationship_type contains "Same" or "PossibleSame"
        // - target is a participant
        // - certainty > some threshold
        vec![]
    }
    
    // Get all interpretations of an entity
    pub fn get_all_interpretations(entity: &Entity) -> Vec<&Property> {
        // Could be in properties["interpretations"]
        // Could be through relationships
        // Could be in quantum states
        vec![]
    }
    
    // Collapse quantum state to classical
    pub fn collapse_superposition(quantum_entity: &Entity) -> Entity {
        // Pick highest probability state
        // Or merge states based on observation
        Entity::new("Collapsed")
    }
}