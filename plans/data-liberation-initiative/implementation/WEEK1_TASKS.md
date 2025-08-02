# Week 1 Implementation Tasks: Data Liberation Foundation

## Overview
Week 1 focuses on building the GRAMPS XML parser and liberation transformer to get data flowing into the ResearchProcess-GPS Layer 1 model.

## Day 1-2: GRAMPS Parser Setup

### Task 1.1: Create Import Crate Structure
```bash
cargo new crates/rp-gramps-import --lib
```

**Crate Dependencies:**
```toml
[dependencies]
quick-xml = "0.30"
serde = { version = "1.0", features = ["derive"] }
serde_xml_rs = "0.6"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.5", features = ["v4", "serde"] }
rp-core = { path = "../rp-core" }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"

[dev-dependencies]
tempfile = "3.8"
pretty_assertions = "1.4"
```

### Task 1.2: Define GRAMPS Data Structures
```rust
// crates/rp-gramps-import/src/models.rs

#[derive(Debug, Deserialize)]
pub struct GrampsDatabase {
    pub header: Header,
    pub events: Option<Events>,
    pub people: Option<People>,
    pub families: Option<Families>,
    pub sources: Option<Sources>,
    pub places: Option<Places>,
    pub citations: Option<Citations>,
    pub repositories: Option<Repositories>,
    pub notes: Option<Notes>,
}

#[derive(Debug, Deserialize)]
pub struct Person {
    #[serde(rename = "@handle")]
    pub handle: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@change")]
    pub change: Option<String>,
    pub gender: Option<Gender>,
    pub name: Vec<Name>,
    #[serde(rename = "eventref")]
    pub event_refs: Option<Vec<EventRef>>,
    #[serde(rename = "citationref")]
    pub citation_refs: Option<Vec<CitationRef>>,
    // ... other fields
}
```

### Task 1.3: Implement XML Parser
```rust
// crates/rp-gramps-import/src/parser.rs

use quick_xml::de::from_str;
use std::fs;
use std::path::Path;

pub struct GrampsParser;

impl GrampsParser {
    pub fn parse_file(path: &Path) -> Result<GrampsDatabase, Error> {
        let xml_content = fs::read_to_string(path)?;
        Self::parse_string(&xml_content)
    }

    pub fn parse_string(xml: &str) -> Result<GrampsDatabase, Error> {
        // Clean and prepare XML
        let cleaned_xml = Self::prepare_xml(xml)?;

        // Parse using quick-xml
        let database: GrampsDatabase = from_str(&cleaned_xml)?;

        // Validate parsed data
        Self::validate_database(&database)?;

        Ok(database)
    }

    fn prepare_xml(xml: &str) -> Result<String, Error> {
        // Handle GRAMPS-specific XML quirks
        // Remove DOCTYPE, handle namespaces, etc.
        Ok(xml.to_string())
    }
}
```

### Task 1.4: Create Test Infrastructure
```rust
// crates/rp-gramps-import/src/test_data.rs

pub fn minimal_gramps_xml() -> &'static str {
    r#"<?xml version="1.0" encoding="UTF-8"?>
    <database>
        <people>
            <person handle="_abc123" id="I0001">
                <gender>M</gender>
                <name type="Birth Name">
                    <first>John</first>
                    <surname>Smith</surname>
                </name>
            </person>
        </people>
    </database>"#
}

pub fn complex_gramps_xml() -> &'static str {
    include_str!("../test_data/complex_family.xml")
}
```

## Day 3-4: Liberation Transformer

### Task 2.1: Define Liberation Context
```rust
// crates/rp-gramps-import/src/liberation/context.rs

use rp_core::prelude::*;
use std::collections::HashMap;

pub struct LiberationContext {
    // Track transformations
    pub gramps_to_identity: HashMap<String, EntityId>,
    pub gramps_to_evidence: HashMap<String, EntityId>,
    pub gramps_to_source: HashMap<String, EntityId>,

    // Collect liberated entities
    pub identities: Vec<IdentityPersona>,
    pub evidence: Vec<Evidence>,
    pub theories: Vec<Theory>,
    pub relationships: Vec<Relationship>,
    pub sources: Vec<Source>,
    pub citations: Vec<Citation>,
    pub locations: Vec<Location>,

    // Research generation
    pub research_questions: Vec<Theory>,
    pub analyses: Vec<Analysis>,
    pub confidence_containers: Vec<Confidence>,

    // Statistics
    pub stats: LiberationStats,
}

#[derive(Debug, Default)]
pub struct LiberationStats {
    pub persons_processed: usize,
    pub identities_created: usize,
    pub evidence_extracted: usize,
    pub questions_generated: usize,
    pub relationships_proposed: usize,
    pub conflicts_detected: usize,
}
```

### Task 2.2: Implement Person Liberation
```rust
// crates/rp-gramps-import/src/liberation/person.rs

pub struct PersonLiberator {
    researcher_id: EntityId,
}

impl PersonLiberator {
    pub fn liberate(
        &self,
        person: &GrampsPerson,
        context: &mut LiberationContext,
    ) -> Result<EntityId> {
        // Create identity in HYPOTHESIS state
        let mut identity = IdentityPersona::new(
            self.extract_primary_name(person),
            self.determine_identity_type(person),
            self.researcher_id,
        );

        // Transition to hypothesis state
        identity.transition(
            IdentityState::Hypothesis,
            self.researcher_id,
            Some(format!(
                "Imported from GRAMPS ID: {}. All facts require verification.",
                person.id
            )),
        ).await?;

        // Add import notes
        identity.notes = Some(self.generate_import_notes(person));

        // Process names for alternatives
        for name in &person.names {
            if !name.is_primary() {
                identity.add_alternative_name(self.format_name(name));
            }
        }

        // Generate research scope
        identity.geographic_scope = self.extract_geographic_scope(person, context);
        identity.temporal_scope = self.extract_temporal_scope(person, context);

        // Store in context
        let identity_id = identity.id();
        context.gramps_to_identity.insert(person.handle.clone(), identity_id);
        context.identities.push(identity);
        context.stats.identities_created += 1;

        // Generate research questions
        let questions = self.generate_person_questions(person, identity_id);
        context.research_questions.extend(questions);

        Ok(identity_id)
    }
}
```

### Task 2.3: Implement Event → Evidence Liberation
```rust
// crates/rp-gramps-import/src/liberation/event.rs

pub struct EventLiberator {
    researcher_id: EntityId,
}

impl EventLiberator {
    pub fn liberate(
        &self,
        event: &GrampsEvent,
        person_handle: Option<&str>,
        context: &mut LiberationContext,
    ) -> Result<EntityId> {
        // Events become Evidence, not Facts!
        let mut evidence = Evidence::new(
            format!("GRAMPS Event: {} ({})", event.event_type, event.id),
            EvidenceType::Extracted,
            None, // Source will be linked via citations
            self.researcher_id,
        );

        // Extract fact-like information
        let extracted_fact = ExtractedFact {
            fact_type: event.event_type.clone(),
            value: self.extract_fact_value(event),
            location: self.extract_location(event, context),
            confidence: 0.0, // Unverified!
        };

        evidence.extracted_facts.push(extracted_fact);

        // Add liberation notes
        evidence.notes = Some(format!(
            "LIBERATION NOTE: This event was imported from GRAMPS and \
             requires source verification before acceptance as fact.\n\
             Original Event ID: {}\n\
             Person Reference: {:?}",
            event.id,
            person_handle
        ));

        // Store in context
        let evidence_id = evidence.id();
        context.gramps_to_evidence.insert(event.handle.clone(), evidence_id);
        context.evidence.push(evidence);
        context.stats.evidence_extracted += 1;

        Ok(evidence_id)
    }
}
```

### Task 2.4: Implement Family → Theory Liberation
```rust
// crates/rp-gramps-import/src/liberation/family.rs

pub struct FamilyLiberator {
    researcher_id: EntityId,
}

impl FamilyLiberator {
    pub fn liberate(
        &self,
        family: &GrampsFamily,
        context: &mut LiberationContext,
    ) -> Result<()> {
        // Create overarching family theory
        let mut family_theory = Theory::new(
            format!("Family Unit {} Verification", family.id),
            "Verify the composition and relationships within this family unit",
            self.researcher_id,
        );

        family_theory.state = TheoryState::Proposed;

        // Add family members to theory
        if let Some(father_handle) = &family.father {
            if let Some(&father_id) = context.gramps_to_identity.get(father_handle) {
                family_theory.supporting_entities.push(father_id);
            }
        }

        if let Some(mother_handle) = &family.mother {
            if let Some(&mother_id) = context.gramps_to_identity.get(mother_handle) {
                family_theory.supporting_entities.push(mother_id);
            }
        }

        // Create PROPOSED relationships
        if let (Some(father_handle), Some(mother_handle)) =
            (&family.father, &family.mother) {

            if let (Some(&father_id), Some(&mother_id)) =
                (context.gramps_to_identity.get(father_handle),
                 context.gramps_to_identity.get(mother_handle)) {

                let mut marriage = Relationship::new(
                    father_id,
                    mother_id,
                    RelationshipType::Partnership("marriage-proposed"),
                    self.researcher_id,
                );

                marriage.state = RelationshipState::Proposed;
                marriage.notes = Some(format!(
                    "IMPORTED: Marriage relationship from GRAMPS family {}. \
                     Requires documentary evidence for verification.",
                    family.id
                ));

                context.relationships.push(marriage);
                context.stats.relationships_proposed += 1;

                // Generate marriage verification question
                let marriage_question = Theory::new(
                    format!("Verify marriage in family {}", family.id),
                    "Locate and verify marriage documentation",
                    self.researcher_id,
                );
                context.research_questions.push(marriage_question);
            }
        }

        // Handle parent-child relationships
        for child_ref in &family.children {
            self.create_parent_child_theories(&family, child_ref, context)?;
        }

        context.theories.push(family_theory);
        Ok(())
    }
}
```

## Day 5-7: Database Integration & Import Pipeline

### Task 3.1: Create Import Command Structure
```rust
// crates/rp-gramps-import/src/import.rs

pub struct GrampsImporter {
    db: Arc<dyn StorageBackend>,
    person_liberator: PersonLiberator,
    event_liberator: EventLiberator,
    family_liberator: FamilyLiberator,
    source_liberator: SourceLiberator,
    question_generator: ResearchQuestionGenerator,
}

impl GrampsImporter {
    pub async fn import_file(
        &self,
        path: &Path,
        options: ImportOptions,
    ) -> Result<ImportReport> {
        info!("Starting GRAMPS import from: {:?}", path);

        // Parse XML
        let gramps_db = GrampsParser::parse_file(path)?;

        // Create liberation context
        let mut context = LiberationContext::new();

        // Phase 1: Sources (needed for citations)
        if let Some(sources) = &gramps_db.sources {
            for source in sources {
                self.source_liberator.liberate(source, &mut context).await?;
            }
        }

        // Phase 2: Places (needed for events)
        if let Some(places) = &gramps_db.places {
            for place in places {
                self.place_liberator.liberate(place, &mut context).await?;
            }
        }

        // Phase 3: People (create identities)
        if let Some(people) = &gramps_db.people {
            for person in people {
                self.person_liberator.liberate(person, &mut context).await?;
            }
        }

        // Phase 4: Events (create evidence)
        if let Some(events) = &gramps_db.events {
            for event in events {
                self.event_liberator.liberate(event, None, &mut context).await?;
            }
        }

        // Phase 5: Families (create theories & relationships)
        if let Some(families) = &gramps_db.families {
            for family in families {
                self.family_liberator.liberate(family, &mut context).await?;
            }
        }

        // Phase 6: Generate research framework
        self.generate_research_framework(&mut context).await?;

        // Phase 7: Persist everything
        self.persist_liberated_data(context).await
    }
}
```

### Task 3.2: Implement Research Question Generation
```rust
// crates/rp-gramps-import/src/questions.rs

pub struct ResearchQuestionGenerator {
    patterns: Vec<QuestionPattern>,
}

impl ResearchQuestionGenerator {
    pub fn generate_for_import(
        &self,
        context: &LiberationContext,
    ) -> Vec<Theory> {
        let mut questions = Vec::new();

        // Identity verification questions
        for identity in &context.identities {
            questions.extend(self.identity_questions(identity));
        }

        // Evidence verification questions
        for evidence in &context.evidence {
            if evidence.source_ref.is_none() {
                questions.push(Theory::new(
                    "Verify evidence source",
                    format!("Locate source documentation for: {}", evidence.description),
                    SYSTEM_RESEARCHER_ID,
                ));
            }
        }

        // Relationship verification questions
        for relationship in &context.relationships {
            questions.push(Theory::new(
                format!("Verify {} relationship", relationship.relationship_type),
                "Find documentary evidence for this relationship",
                SYSTEM_RESEARCHER_ID,
            ));
        }

        // Gap analysis questions
        questions.extend(self.analyze_gaps(context));

        questions
    }

    fn analyze_gaps(&self, context: &LiberationContext) -> Vec<Theory> {
        let mut gap_questions = Vec::new();

        // Find people without birth evidence
        for identity in &context.identities {
            let has_birth = context.evidence.iter().any(|e|
                e.extracted_facts.iter().any(|f| f.fact_type == "Birth")
            );

            if !has_birth {
                gap_questions.push(Theory::new(
                    format!("Find birth record for {}", identity.primary_name),
                    "No birth evidence was imported - locate primary sources",
                    SYSTEM_RESEARCHER_ID,
                ));
            }
        }

        gap_questions
    }
}
```

### Task 3.3: Create Import Transaction Management
```rust
// crates/rp-gramps-import/src/persistence.rs

impl GrampsImporter {
    async fn persist_liberated_data(
        &self,
        context: LiberationContext,
    ) -> Result<ImportReport> {
        let mut transaction = self.db.begin_transaction().await?;

        try {
            // Persist sources first (referenced by others)
            for source in context.sources {
                self.db.create_entity(&source, &mut transaction).await?;
            }

            // Persist locations
            for location in context.locations {
                self.db.create_entity(&location, &mut transaction).await?;
            }

            // Persist identities
            for identity in context.identities {
                self.db.create_entity(&identity, &mut transaction).await?;
            }

            // Persist evidence
            for evidence in context.evidence {
                self.db.create_entity(&evidence, &mut transaction).await?;
            }

            // Persist theories
            for theory in context.theories {
                self.db.create_entity(&theory, &mut transaction).await?;
            }

            // Persist relationships
            for relationship in context.relationships {
                self.db.create_entity(&relationship, &mut transaction).await?;
            }

            // Persist research questions
            for question in context.research_questions {
                self.db.create_entity(&question, &mut transaction).await?;
            }

            // Commit transaction
            transaction.commit().await?;

            Ok(ImportReport {
                success: true,
                stats: context.stats,
                warnings: vec![],
                errors: vec![],
            })
        } catch (e) {
            transaction.rollback().await?;
            Err(e)
        }
    }
}
```

### Task 3.4: Create CLI Import Tool
```rust
// crates/rp-gramps-import/src/bin/import.rs

use clap::Parser;
use rp_gramps_import::{GrampsImporter, ImportOptions};

#[derive(Parser)]
#[command(name = "rp-import-gramps")]
#[command(about = "Import GRAMPS XML into ResearchProcess-GPS")]
struct Args {
    /// Path to GRAMPS XML file
    #[arg(short, long)]
    file: PathBuf,

    /// Database connection string
    #[arg(short, long, env = "DATABASE_URL")]
    database: String,

    /// Researcher name for attribution
    #[arg(short, long)]
    researcher: String,

    /// Generate detailed import report
    #[arg(long)]
    detailed_report: bool,

    /// Dry run - parse but don't import
    #[arg(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Connect to database
    let db = connect_database(&args.database).await?;

    // Create importer
    let importer = GrampsImporter::new(db, &args.researcher)?;

    // Run import
    let report = if args.dry_run {
        importer.analyze_file(&args.file).await?
    } else {
        importer.import_file(&args.file, ImportOptions::default()).await?
    };

    // Display report
    println!("{}", format_import_report(&report, args.detailed_report));

    Ok(())
}
```

## Testing & Validation

### Integration Tests
```rust
// crates/rp-gramps-import/tests/integration.rs

#[tokio::test]
async fn test_minimal_import() {
    let xml = r#"
    <database>
        <people>
            <person handle="_p1" id="I0001">
                <name type="Birth Name">
                    <first>Test</first>
                    <surname>Person</surname>
                </name>
            </person>
        </people>
    </database>"#;

    let importer = create_test_importer().await;
    let report = importer.import_string(xml).await.unwrap();

    assert_eq!(report.stats.persons_processed, 1);
    assert_eq!(report.stats.identities_created, 1);
    assert!(report.stats.questions_generated > 0);
}

#[tokio::test]
async fn test_complex_family_import() {
    let xml = include_str!("test_data/complex_family.xml");

    let importer = create_test_importer().await;
    let report = importer.import_string(xml).await.unwrap();

    // Verify liberation happened
    assert!(report.stats.identities_created > report.stats.persons_processed);
    assert!(report.stats.relationships_proposed > 0);
    assert!(report.stats.questions_generated > report.stats.identities_created);
}
```

## Week 1 Deliverables

1. **Working GRAMPS XML Parser**
   - Handles real GRAMPS exports
   - Robust error handling
   - Comprehensive test coverage

2. **Liberation Transformer**
   - Converts persons → identity hypotheses
   - Transforms events → floating evidence
   - Creates theories from families
   - Generates research questions

3. **Database Integration**
   - Transactional imports
   - Progress tracking
   - Error recovery

4. **CLI Import Tool**
   - Simple command-line interface
   - Import reports
   - Dry-run capability

5. **Documentation**
   - Liberation mapping rules
   - Import guide
   - Architecture documentation

## Success Criteria

- [ ] Import 1000+ person GRAMPS file in <30 seconds
- [ ] 100% of GRAMPS data preserved in liberated form
- [ ] Every import generates research questions
- [ ] All imports are transactional (succeed or rollback)
- [ ] Clear liberation report showing transformations

---

By end of Week 1, we'll have data flowing into the system and can begin Week 2's visual interface work.