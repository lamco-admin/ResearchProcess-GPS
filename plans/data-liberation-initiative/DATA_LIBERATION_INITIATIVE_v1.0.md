# Data Liberation Initiative - ResearchProcess-GPS
## Transforming Genealogical Data from Conclusions to Research Questions
### Version: 1.0 - Initial Draft
### Date: 2025-08-01
### Status: DRAFT - Rapid Prototyping Plan

---

## Executive Summary

The Data Liberation Initiative is a focused, rapid-prototyping effort to demonstrate the revolutionary potential of ResearchProcess-GPS by:
1. Importing traditional genealogical data (starting with GRAMPS XML)
2. Transforming conclusional data into research-oriented hypotheses
3. Providing minimal visual interfaces to manipulate liberated data
4. Demonstrating unique capabilities impossible in traditional systems

**Core Thesis**: Traditional genealogy software imprisons data in conclusional formats. ResearchProcess-GPS liberates this data into a fluid, research-oriented model where everything is a hypothesis to be explored.

---

## Vision: Data Liberation

### The Imprisonment Problem
Traditional genealogy software (including GRAMPS) forces data into rigid structures:
- A person IS John Smith (conclusion)
- John Smith WAS BORN on 1 Jan 1850 (fact)
- John Smith MARRIED Mary Jones (relationship)
- The 1850 census PROVES John's age (evidence locked to person)

### The Liberation Solution
ResearchProcess-GPS transforms this into:
- An identity that MIGHT BE John Smith (hypothesis)
- Evidence SUGGESTS someone was born around 1850 (evidence)
- Two identities MAY HAVE married (theory)
- The 1850 census COULD REFER to multiple people (floating evidence)

### Why This Matters
1. **Research Reality**: Professional genealogists know nothing is certain
2. **Evidence Fluidity**: Same evidence can support multiple theories
3. **Identity Uncertainty**: One name might be multiple people (or vice versa)
4. **Theory Exploration**: Test "what if" scenarios impossible in rigid systems

---

## Phase 1: GRAMPS Data Liberation (Week 1)

### 1.1 Understanding GRAMPS XML Structure

GRAMPS XML contains several key sections that map to our Layer 1 entities:

```xml
<database>
  <people>
    <person handle="_abc123" change="1234567890" id="I0001">
      <gender>M</gender>
      <name type="Birth Name">
        <first>John</first>
        <surname>Smith</surname>
      </name>
      <eventref hlink="_event1" role="Primary"/>
      <citationref hlink="_citation1"/>
    </person>
  </people>
  
  <families>
    <family handle="_fam1" change="1234567890" id="F0001">
      <father hlink="_abc123"/>
      <mother hlink="_def456"/>
      <childref hlink="_ghi789"/>
    </family>
  </families>
  
  <events>
    <event handle="_event1" change="1234567890" id="E0001">
      <type>Birth</type>
      <dateval val="1850-01-01"/>
      <place hlink="_place1"/>
    </event>
  </events>
  
  <sources>
    <source handle="_src1" change="1234567890" id="S0001">
      <stitle>1850 United States Federal Census</stitle>
    </source>
  </sources>
</database>
```

### 1.2 Liberation Transformation Rules

#### People → IdentityPersona (HYPOTHESIS state)
```rust
// Transformation logic
fn liberate_person(gramps_person: GrampsPerson) -> IdentityPersona {
    let mut identity = IdentityPersona::new(
        extract_primary_name(&gramps_person),
        IdentityType::Named,
        IMPORT_RESEARCHER_ID,
    );
    
    // Critical: Start as HYPOTHESIS, not CONCLUDED
    identity.transition(
        IdentityState::Hypothesis,
        IMPORT_RESEARCHER_ID,
        Some(format!(
            "Imported from GRAMPS ID: {}. Requires verification.",
            gramps_person.id
        ))
    ).await?;
    
    // Generate research notes
    identity.notes = Some(format!(
        "LIBERATION NOTES:\n\
        - Original GRAMPS ID: {}\n\
        - Import Date: {}\n\
        - Certainty: UNVERIFIED\n\
        - Research Needed: Verify all relationships and facts\n\
        - Alternative Identities: Not yet explored",
        gramps_person.id,
        Utc::now()
    ));
    
    identity
}
```

#### Events → Evidence (not Facts!)
```rust
fn liberate_event(gramps_event: GrampsEvent, person_handle: &str) -> Evidence {
    Evidence::new(
        format!("GRAMPS Event: {} for person {}", gramps_event.type, person_handle),
        EvidenceType::Extracted,
        None, // No source yet - will link separately
        IMPORT_RESEARCHER_ID,
    )
}
```

#### Families → Proposed Relationships + Research Questions
```rust
fn liberate_family(family: GrampsFamily) -> (Vec<Theory>, Vec<Relationship>) {
    let mut theories = Vec::new();
    let mut relationships = Vec::new();
    
    // Create a theory about this family unit
    let family_theory = Theory::new(
        format!("Family Unit {} Verification", family.id),
        format!("Verify that these individuals actually formed a family unit"),
        IMPORT_RESEARCHER_ID,
    );
    
    // Create PROPOSED relationships (not concluded!)
    if let (Some(father), Some(mother)) = (family.father, family.mother) {
        let marriage_rel = Relationship::new(
            father,
            mother,
            RelationshipType::Partnership("marriage-proposed"),
            IMPORT_RESEARCHER_ID,
        );
        marriage_rel.state = RelationshipState::Proposed;
        relationships.push(marriage_rel);
        
        // Generate research question
        theories.push(Theory::new(
            "Verify Marriage",
            format!("Find evidence for marriage between {} and {}", father, mother),
            IMPORT_RESEARCHER_ID,
        ));
    }
    
    (theories, relationships)
}
```

### 1.3 Research Question Generation

For every piece of imported data, generate research questions:

```rust
struct ResearchQuestionGenerator {
    patterns: Vec<QuestionPattern>,
}

impl ResearchQuestionGenerator {
    fn generate_for_identity(&self, identity: &IdentityPersona) -> Vec<Theory> {
        let mut questions = Vec::new();
        
        // Identity verification
        questions.push(Theory::new(
            format!("Verify {}", identity.primary_name),
            "Confirm this person's identity through documentary evidence",
            IMPORT_RESEARCHER_ID,
        ));
        
        // Missing evidence detection
        if identity.evidence_references.is_empty() {
            questions.push(Theory::new(
                format!("Find evidence for {}", identity.primary_name),
                "No direct evidence imported - locate primary sources",
                IMPORT_RESEARCHER_ID,
            ));
        }
        
        // Name variation research
        questions.push(Theory::new(
            format!("Name variations for {}", identity.primary_name),
            "Research possible name variations, nicknames, or aliases",
            IMPORT_RESEARCHER_ID,
        ));
        
        questions
    }
    
    fn generate_for_relationship(&self, rel: &Relationship) -> Vec<Theory> {
        vec![
            Theory::new(
                "Verify Relationship",
                format!("Find documentary evidence for this {} relationship", rel.relationship_type),
                IMPORT_RESEARCHER_ID,
            ),
            Theory::new(
                "Relationship Timeline",
                "Establish when this relationship began and ended",
                IMPORT_RESEARCHER_ID,
            ),
        ]
    }
    
    fn generate_for_evidence_gaps(&self, identities: &[IdentityPersona]) -> Vec<Theory> {
        // Detect patterns and gaps
        let mut gap_questions = Vec::new();
        
        // Check for missing vital records
        for identity in identities {
            let has_birth = false; // Check evidence
            let has_death = false; // Check evidence
            
            if !has_birth {
                gap_questions.push(Theory::new(
                    format!("Find birth record for {}", identity.primary_name),
                    "Locate birth or baptism record",
                    IMPORT_RESEARCHER_ID,
                ));
            }
        }
        
        gap_questions
    }
}
```

### 1.4 Import Process Architecture

```rust
pub struct GrampsImporter {
    db: Box<dyn StorageBackend>,
    question_generator: ResearchQuestionGenerator,
    liberation_stats: LiberationStats,
}

pub struct LiberationStats {
    pub persons_liberated: usize,
    pub identities_created: usize,
    pub evidence_extracted: usize,
    pub questions_generated: usize,
    pub relationships_proposed: usize,
    pub theories_created: usize,
}

impl GrampsImporter {
    pub async fn import_file(&mut self, path: &Path) -> Result<LiberationReport> {
        // 1. Parse GRAMPS XML
        let gramps_data = parse_gramps_xml(path)?;
        
        // 2. Create liberation context
        let mut context = LiberationContext::new();
        
        // 3. Liberation transformation
        for person in gramps_data.people {
            let identity = self.liberate_person(person, &mut context).await?;
            context.identities.push(identity);
        }
        
        // 4. Generate research framework
        context.theories = self.question_generator.generate_all(&context);
        
        // 5. Persist liberated data
        self.persist_liberation(context).await?;
        
        // 6. Generate report
        Ok(self.create_liberation_report())
    }
}
```

---

## Phase 2: Minimal Visual Workspace (Week 2)

### 2.1 Core UI Concepts

The workspace must visualize the liberated, fluid nature of the data:

```typescript
interface WorkspaceComponents {
  // Identity Cloud - not a family tree!
  identityCloud: {
    display: "force-directed graph" | "card grid" | "timeline";
    interactions: ["drag", "merge", "split", "connect"];
    states: ["hypothesis", "working", "concluded"];
  };
  
  // Evidence Pool - floating evidence
  evidencePool: {
    display: "list" | "cards" | "timeline";
    interactions: ["drag-to-identity", "correlate", "analyze"];
    filters: ["type", "date", "location", "confidence"];
  };
  
  // Theory Workshop - active research
  theoryWorkshop: {
    display: "kanban" | "tree" | "comparison";
    interactions: ["create", "branch", "merge", "test"];
    tools: ["evidence-assignment", "identity-linking", "timeline-building"];
  };
  
  // Research Questions - generated & manual
  questionQueue: {
    display: "priority-list" | "mind-map" | "categorized";
    interactions: ["assign", "investigate", "answer", "spawn-sub-questions"];
    automation: ["auto-generate", "suggest-next", "detect-gaps"];
  };
}
```

### 2.2 Identity Manipulation Interface

```rust
// Leptos components for identity manipulation
#[component]
fn IdentityCloudView(cx: Scope, identities: Signal<Vec<IdentityPersona>>) -> impl IntoView {
    view! { cx,
        <div class="identity-cloud">
            <IdentityControls />
            <ForceGraph3D
                data=move || create_graph_data(identities.get())
                on_node_click=|node| handle_identity_click(node)
                on_node_drag_end=|node, pos| handle_identity_move(node, pos)
            />
            <IdentitySplitModal />
            <IdentityMergeModal />
        </div>
    }
}

#[component]
fn IdentityCard(
    cx: Scope, 
    identity: IdentityPersona,
    on_split: Callback<SplitRequest>,
    on_merge: Callback<EntityId>,
) -> impl IntoView {
    let (expanded, set_expanded) = create_signal(cx, false);
    
    view! { cx,
        <div class=format!("identity-card state-{:?}", identity.state)>
            <div class="header" on:click=move |_| set_expanded.update(|e| *e = !*e)>
                <h3>{identity.primary_name}</h3>
                <StateIndicator state=identity.state />
                <ConfidenceIndicator confidence=identity.confidence_refs.len() />
            </div>
            
            <Show when=expanded fallback=|_| view! { cx, <></> }>
                <div class="details">
                    <EvidenceList evidence=identity.evidence_references />
                    <TheoryList theories=identity.theory_refs />
                    
                    <div class="actions">
                        <button on:click=move |_| on_split(create_split_request(&identity))>
                            "Split Identity"
                        </button>
                        <button on:click=move |_| start_merge_mode(identity.id())>
                            "Merge With..."
                        </button>
                        <button on:click=move |_| change_state(&identity)>
                            "Change State"
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    }
}
```

### 2.3 Evidence Floating System

Evidence isn't attached to people - it floats between theories:

```rust
#[component]
fn EvidencePool(cx: Scope, evidence: Signal<Vec<Evidence>>) -> impl IntoView {
    let (drag_state, set_drag_state) = create_signal(cx, DragState::None);
    
    view! { cx,
        <div class="evidence-pool">
            <h2>"Unassigned Evidence"</h2>
            <For
                each=move || evidence.get().into_iter().filter(|e| !e.is_assigned())
                key=|e| e.id()
                view=move |cx, evidence_item| {
                    view! { cx,
                        <EvidenceCard
                            evidence=evidence_item
                            draggable=true
                            on_drag_start=move |_| set_drag_state(DragState::Evidence(evidence_item.id()))
                        />
                    }
                }
            />
        </div>
    }
}

#[component]
fn EvidenceAssignmentZone(cx: Scope, identity: IdentityPersona) -> impl IntoView {
    view! { cx,
        <div 
            class="assignment-zone"
            on:dragover=|e| e.prevent_default()
            on:drop=move |e| {
                e.prevent_default();
                if let Some(evidence_id) = get_dragged_evidence_id() {
                    assign_evidence_to_identity(evidence_id, identity.id());
                }
            }
        >
            "Drop evidence here to create association"
        </div>
    }
}
```

### 2.4 Theory Building Interface

```rust
#[component]
fn TheoryWorkshop(cx: Scope, active_theory: Signal<Option<Theory>>) -> impl IntoView {
    view! { cx,
        <div class="theory-workshop">
            <TheorySelector on_select=set_active_theory />
            
            <Show
                when=move || active_theory.get().is_some()
                fallback=|cx| view! { cx, <TheoryCreator /> }
            >
                {move || {
                    let theory = active_theory.get().unwrap();
                    view! { cx,
                        <div class="theory-workspace">
                            <h2>{theory.question}</h2>
                            <TheoryTimeline theory=theory.clone() />
                            <IdentityAssignments theory=theory.clone() />
                            <EvidenceCorrelation theory=theory.clone() />
                            <TheoryComparison base=theory.clone() />
                        </div>
                    }
                }}
            </Show>
        </div>
    }
}
```

---

## Phase 3: Research Question Module (Week 3)

### 3.1 Module Architecture

```rust
// modules/research-question/src/lib.rs
pub struct ResearchQuestionModule {
    analyzer: QuestionAnalyzer,
    generator: QuestionGenerator,
    prioritizer: QuestionPrioritizer,
    tracker: ProgressTracker,
}

#[async_trait]
impl Module for ResearchQuestionModule {
    fn metadata(&self) -> ModuleMetadata {
        ModuleMetadata {
            name: "Research Question Manager",
            version: "0.1.0",
            description: "Generates and manages research questions from imported data",
            capabilities: vec![
                Capability::GenerateQuestions,
                Capability::TrackProgress,
                Capability::SuggestNextSteps,
                Capability::AnalyzeGaps,
            ],
        }
    }
    
    async fn handle_command(&self, cmd: ModuleCommand) -> Result<ModuleResponse> {
        match cmd {
            ModuleCommand::AnalyzeImport { data } => {
                let questions = self.analyze_imported_data(data).await?;
                Ok(ModuleResponse::QuestionsGenerated { questions })
            },
            ModuleCommand::PrioritizeQuestions { questions } => {
                let prioritized = self.prioritizer.prioritize(questions).await?;
                Ok(ModuleResponse::PrioritizedList { questions: prioritized })
            },
            ModuleCommand::SuggestNext { context } => {
                let suggestion = self.suggest_next_research(context).await?;
                Ok(ModuleResponse::NextSteps { suggestion })
            },
            _ => Err(Error::UnsupportedCommand),
        }
    }
}
```

### 3.2 Question Generation Patterns

```rust
pub struct QuestionPattern {
    pub trigger: PatternTrigger,
    pub template: String,
    pub priority: Priority,
    pub category: QuestionCategory,
}

pub enum PatternTrigger {
    MissingEvidence { expected_type: String },
    ConflictingEvidence { conflict_type: String },
    UnverifiedRelationship { relationship_type: String },
    IdentityUncertainty { uncertainty_type: String },
    TimelineGap { years: Range<i32> },
    GeographicAnomaly { distance: f64 },
}

impl QuestionGenerator {
    fn generate_from_patterns(&self, context: &ResearchContext) -> Vec<Theory> {
        let mut questions = Vec::new();
        
        // Identity verification patterns
        for identity in &context.identities {
            if identity.evidence_references.len() < 3 {
                questions.push(self.create_question(
                    PatternTrigger::MissingEvidence { 
                        expected_type: "primary".to_string() 
                    },
                    identity,
                ));
            }
            
            // Check for name variations
            if identity.alternative_names.is_empty() {
                questions.push(Theory::new(
                    format!("Research name variations for {}", identity.primary_name),
                    "Investigate nicknames, spelling variations, and cultural naming patterns",
                    SYSTEM_RESEARCHER_ID,
                ));
            }
        }
        
        // Relationship verification patterns
        for relationship in &context.relationships {
            if relationship.state == RelationshipState::Proposed {
                questions.push(Theory::new(
                    "Verify proposed relationship",
                    format!("Find documentary evidence for {} relationship", 
                        relationship.relationship_type),
                    SYSTEM_RESEARCHER_ID,
                ));
            }
        }
        
        // Timeline analysis patterns
        let timeline_gaps = self.analyze_timeline_gaps(&context.evidence);
        for gap in timeline_gaps {
            questions.push(Theory::new(
                format!("Fill timeline gap {}-{}", gap.start, gap.end),
                "Research this period to find missing life events",
                SYSTEM_RESEARCHER_ID,
            ));
        }
        
        questions
    }
}
```

### 3.3 Research Progress Tracking

```rust
pub struct ProgressTracker {
    question_states: HashMap<EntityId, QuestionState>,
    research_sessions: Vec<ResearchSession>,
    findings: HashMap<EntityId, Vec<Finding>>,
}

#[derive(Debug, Clone)]
pub enum QuestionState {
    Generated { date: DateTime<Utc> },
    Assigned { researcher: EntityId, date: DateTime<Utc> },
    InProgress { 
        researcher: EntityId,
        started: DateTime<Utc>,
        sessions: Vec<EntityId>,
    },
    Blocked { reason: String, date: DateTime<Utc> },
    Answered { 
        conclusion: String,
        evidence: Vec<EntityId>,
        date: DateTime<Utc>,
    },
    Spawned { 
        new_questions: Vec<EntityId>,
        reason: String,
    },
}

impl ProgressTracker {
    pub fn update_question_state(
        &mut self, 
        question_id: EntityId, 
        new_state: QuestionState
    ) -> Result<()> {
        // Track state transitions
        let previous = self.question_states.get(&question_id);
        
        // Validate transition
        match (&previous, &new_state) {
            (Some(QuestionState::Answered { .. }), _) => {
                return Err(Error::InvalidTransition("Cannot modify answered question"));
            },
            _ => {},
        }
        
        self.question_states.insert(question_id, new_state);
        Ok(())
    }
    
    pub fn get_next_priority_question(&self) -> Option<EntityId> {
        // Smart prioritization based on:
        // - Dependencies (some questions must be answered first)
        // - Research momentum (continue in same area)
        // - Available evidence
        // - Researcher expertise
        
        self.question_states
            .iter()
            .filter(|(_, state)| matches!(state, QuestionState::Generated { .. }))
            .min_by_key(|(id, _)| self.calculate_priority(id))
            .map(|(id, _)| *id)
    }
}
```

---

## Phase 4: Data Manipulation Experiments (Week 4)

### 4.1 Identity Operations

#### Identity Splitting
```rust
pub trait IdentitySplitting {
    /// Split one identity into multiple based on evidence analysis
    fn split_identity(
        &mut self,
        identity: &IdentityPersona,
        split_points: Vec<SplitPoint>,
    ) -> Result<Vec<IdentityPersona>>;
    
    /// Split based on time periods
    fn split_by_timeline(
        &mut self,
        identity: &IdentityPersona,
        dates: Vec<DateTime<Utc>>,
    ) -> Result<Vec<IdentityPersona>>;
    
    /// Split based on geographic locations
    fn split_by_location(
        &mut self,
        identity: &IdentityPersona,
        locations: Vec<Location>,
    ) -> Result<Vec<IdentityPersona>>;
}

pub struct SplitPoint {
    pub criteria: SplitCriteria,
    pub evidence: Vec<EntityId>,
    pub confidence: f32,
}

pub enum SplitCriteria {
    Temporal { before: DateTime<Utc>, after: DateTime<Utc> },
    Geographic { location: Location, radius_km: f64 },
    Evidential { conflicting_evidence: Vec<EntityId> },
    Theoretical { hypothesis: String },
}

impl IdentitySplitting for WorkspaceOperations {
    fn split_identity(
        &mut self,
        identity: &IdentityPersona,
        split_points: Vec<SplitPoint>,
    ) -> Result<Vec<IdentityPersona>> {
        let mut new_identities = Vec::new();
        
        for (i, split) in split_points.iter().enumerate() {
            let mut new_identity = identity.clone();
            new_identity.metadata.id = EntityId::new();
            new_identity.primary_name = format!("{} [Split {}]", identity.primary_name, i + 1);
            
            // Distribute evidence based on split criteria
            new_identity.evidence_references = identity.evidence_references
                .iter()
                .filter(|e| split.applies_to_evidence(e))
                .cloned()
                .collect();
            
            // Reset to hypothesis state
            new_identity.state = IdentityState::Hypothesis;
            new_identity.notes = Some(format!(
                "Split from {} based on: {:?}\nOriginal ID: {}",
                identity.primary_name,
                split.criteria,
                identity.id()
            ));
            
            new_identities.push(new_identity);
        }
        
        // Create research questions about the split
        self.create_split_verification_questions(&identity, &new_identities)?;
        
        Ok(new_identities)
    }
}
```

#### Identity Merging
```rust
pub trait IdentityMerging {
    /// Merge multiple identities into one
    fn merge_identities(
        &mut self,
        identities: Vec<&IdentityPersona>,
        merge_strategy: MergeStrategy,
    ) -> Result<IdentityPersona>;
    
    /// Test merge without committing
    fn preview_merge(
        &self,
        identities: Vec<&IdentityPersona>,
    ) -> MergePreview;
}

pub enum MergeStrategy {
    /// Take all evidence from all identities
    UnionAll,
    /// Only take non-conflicting evidence
    ConflictAvoidance,
    /// Prefer evidence from primary identity
    PrimaryPreference { primary: EntityId },
    /// Manual selection of evidence
    Manual { selections: HashMap<EntityId, Vec<EntityId>> },
}

pub struct MergePreview {
    pub combined_evidence: Vec<Evidence>,
    pub conflicts: Vec<EvidenceConflict>,
    pub name_variations: Vec<String>,
    pub timeline: Timeline,
    pub confidence_impact: ConfidenceChange,
}
```

### 4.2 Evidence Operations

#### Evidence Floating
```rust
pub trait EvidenceFloating {
    /// Allow evidence to support multiple theories simultaneously
    fn float_evidence(
        &mut self,
        evidence: &Evidence,
        theories: Vec<EntityId>,
    ) -> Result<()>;
    
    /// Correlate evidence across multiple identities
    fn correlate_evidence(
        &self,
        evidence_items: Vec<&Evidence>,
    ) -> CorrelationMatrix;
    
    /// Find all possible interpretations of evidence
    fn find_interpretations(
        &self,
        evidence: &Evidence,
        context: &ResearchContext,
    ) -> Vec<EvidenceInterpretation>;
}

pub struct EvidenceInterpretation {
    pub supports_identity: Option<EntityId>,
    pub supports_theory: Option<EntityId>,
    pub interpretation: String,
    pub confidence: f32,
    pub reasoning: String,
}

impl EvidenceFloating for WorkspaceOperations {
    fn float_evidence(
        &mut self,
        evidence: &Evidence,
        theories: Vec<EntityId>,
    ) -> Result<()> {
        // Evidence can simultaneously support multiple theories
        for theory_id in theories {
            let theory = self.get_theory_mut(theory_id)?;
            theory.add_evidence(evidence.id());
            
            // Create analysis point for this interpretation
            let analysis = Analysis::new(
                format!("Evidence interpretation for {}", theory.question),
                AnalysisType::EvidenceCorrelation,
                SYSTEM_ANALYST_ID,
            );
            
            theory.add_analysis(analysis);
        }
        
        Ok(())
    }
}
```

### 4.3 Theory Operations

#### Theory Branching
```rust
pub trait TheoryBranching {
    /// Create alternative theory branches
    fn branch_theory(
        &mut self,
        base_theory: &Theory,
        branch_point: BranchPoint,
    ) -> Result<Theory>;
    
    /// Compare multiple theory branches
    fn compare_theories(
        &self,
        theories: Vec<&Theory>,
    ) -> TheoryComparison;
    
    /// Merge theory branches
    fn merge_theories(
        &mut self,
        primary: &Theory,
        secondary: &Theory,
        merge_strategy: TheoryMergeStrategy,
    ) -> Result<Theory>;
}

pub struct BranchPoint {
    pub divergence: String,
    pub alternative_hypothesis: String,
    pub supporting_evidence: Vec<EntityId>,
    pub conflicting_evidence: Vec<EntityId>,
}

pub struct TheoryComparison {
    pub common_evidence: Vec<EntityId>,
    pub unique_evidence: HashMap<EntityId, Vec<EntityId>>,
    pub conflicting_conclusions: Vec<Conflict>,
    pub compatibility_score: f32,
}
```

### 4.4 Revolutionary Capabilities Demo

```rust
/// Demonstrate capabilities impossible in traditional systems
pub struct RevolutionaryDemo {
    workspace: Workspace,
}

impl RevolutionaryDemo {
    /// Show how one GRAMPS person becomes multiple hypothetical identities
    pub async fn demo_identity_explosion(&mut self) -> Result<()> {
        // Import one "John Smith" from GRAMPS
        let gramps_john = self.import_person("John Smith")?;
        
        // Split into multiple possibilities
        let johns = vec![
            self.create_hypothesis("John Smith Sr.", "The father"),
            self.create_hypothesis("John Smith Jr.", "The son"),
            self.create_hypothesis("John Smith of Boston", "Different person?"),
            self.create_hypothesis("Johann Schmidt", "German immigrant?"),
        ];
        
        // Float evidence between them
        for evidence in gramps_john.evidence {
            self.evaluate_evidence_for_all(&evidence, &johns)?;
        }
        
        // Generate research questions
        self.generate_identity_questions(&johns)?;
        
        println!("One GRAMPS person → {} hypothetical identities", johns.len());
        Ok(())
    }
    
    /// Show evidence floating between theories
    pub async fn demo_evidence_floating(&mut self) -> Result<()> {
        let census_1850 = self.get_evidence("1850 Census - John Smith")?;
        
        // Traditional: Census locked to one person
        // Revolutionary: Census might refer to multiple people
        
        let theories = vec![
            self.create_theory("Same John throughout life"),
            self.create_theory("Father and son with same name"),
            self.create_theory("Unrelated men with common name"),
        ];
        
        // Evidence supports all theories differently
        for theory in &theories {
            self.assign_evidence_with_interpretation(
                &census_1850,
                theory,
                "Different interpretation for each theory"
            )?;
        }
        
        Ok(())
    }
    
    /// Show theory branching and comparison
    pub async fn demo_theory_evolution(&mut self) -> Result<()> {
        let base_theory = self.create_theory("Smith Family of Virginia");
        
        // Branch at crucial decision point
        let theory_a = self.branch_theory(&base_theory, BranchPoint {
            divergence: "Death of John Smith".to_string(),
            alternative_hypothesis: "John died in 1853".to_string(),
            supporting_evidence: vec![/* death record */],
            conflicting_evidence: vec![/* 1860 census? */],
        })?;
        
        let theory_b = self.branch_theory(&base_theory, BranchPoint {
            divergence: "Death of John Smith".to_string(),
            alternative_hypothesis: "John moved west in 1853".to_string(),
            supporting_evidence: vec![/* western records */],
            conflicting_evidence: vec![/* death record? */],
        })?;
        
        // Compare theories
        let comparison = self.compare_theories(vec![&theory_a, &theory_b])?;
        
        println!("Theory A confidence: {}", theory_a.calculate_confidence());
        println!("Theory B confidence: {}", theory_b.calculate_confidence());
        println!("Conflicts found: {}", comparison.conflicting_conclusions.len());
        
        Ok(())
    }
}
```

---

## Implementation Schedule

### Week 1: Data Liberation Foundation
**Days 1-2: GRAMPS Parser**
- Set up crate structure
- Implement XML parsing
- Create intermediate data structures
- Write basic tests

**Days 3-4: Liberation Transformer**
- Implement transformation rules
- Create research question generator
- Build liberation statistics
- Test with sample GRAMPS files

**Days 5-7: Database Integration**
- Connect to PostgreSQL
- Implement batch imports
- Create import transaction handling
- Generate liberation reports

**Milestone**: Successfully import GRAMPS file and see liberated data in database

### Week 2: Visual Workspace
**Days 1-2: Web Framework Setup**
- Set up Leptos project
- Create basic layout
- Implement API endpoints
- Set up WebSocket for real-time

**Days 3-4: Identity Visualization**
- Create identity cards
- Implement drag-and-drop
- Build force-directed graph
- Add state change controls

**Days 5-7: Evidence and Theory UI**
- Create evidence pool
- Implement evidence assignment
- Build theory workspace
- Add research question queue

**Milestone**: Interact with liberated data visually

### Week 3: Research Module
**Days 1-3: Module Architecture**
- Create module structure
- Implement module traits
- Build communication protocol
- Test module loading

**Days 4-5: Question Generation**
- Implement pattern matching
- Create question templates
- Build prioritization logic
- Test with real data

**Days 6-7: Progress Tracking**
- Create progress tracker
- Implement state machine
- Build reporting system
- Create module UI

**Milestone**: Working research question module

### Week 4: Advanced Experiments
**Days 1-2: Identity Operations**
- Implement splitting
- Create merging logic
- Test with complex cases

**Days 3-4: Evidence Floating**
- Build evidence correlation
- Create interpretation system
- Test multi-theory support

**Days 5-6: Theory Branching**
- Implement branching logic
- Create comparison tools
- Build merge capabilities

**Day 7: Demo Preparation**
- Create demo scenarios
- Document capabilities
- Prepare presentation

**Milestone**: Demonstrate revolutionary capabilities

---

## Success Metrics

### Technical Metrics
1. **Import Success**: 100% of GRAMPS data preserved
2. **Transformation Accuracy**: All entities correctly mapped
3. **Performance**: <30 seconds for 10,000 person import
4. **UI Responsiveness**: <100ms for drag operations

### Liberation Metrics
1. **Hypothesis Generation**: Every import creates research questions
2. **Evidence Freedom**: 100% of evidence can float between theories
3. **Identity Flexibility**: Any identity can be split/merged
4. **Theory Branching**: Unlimited theory exploration

### User Experience Metrics
1. **Time to First Insight**: <5 minutes from import
2. **Intuitive Operations**: Users understand liberation concept
3. **Visual Clarity**: Data relationships immediately apparent
4. **Research Value**: Clear advantage over traditional tools

---

## Risk Mitigation

### Technical Risks
1. **GRAMPS Format Complexity**
   - Mitigation: Start with subset, expand gradually
   - Fallback: Create simplified import format

2. **Performance with Large Data**
   - Mitigation: Implement pagination and lazy loading
   - Fallback: Import size limits initially

3. **UI Complexity**
   - Mitigation: Progressive disclosure of features
   - Fallback: Start with CLI tools

### Conceptual Risks
1. **User Confusion**
   - Mitigation: Extensive onboarding and tutorials
   - Fallback: Traditional view mode option

2. **Data Liberation Too Radical**
   - Mitigation: Gradual introduction of concepts
   - Fallback: Allow conclusional mode

3. **Research Question Overload**
   - Mitigation: Smart prioritization and filtering
   - Fallback: Manual question management

---

## Future Expansions

### Data Sources
1. FamilySearch API integration
2. Ancestry.com GEDCOM import
3. DNA data incorporation
4. Historical record databases

### Advanced Features
1. AI-powered question generation
2. Pattern recognition across families
3. Automated evidence correlation
4. Collaborative research spaces

### Platform Extensions
1. Mobile field research app
2. Archive integration plugins
3. DNA analysis modules
4. Publication generators

---

## Conclusion

The Data Liberation Initiative represents a fundamental shift in how genealogical data is conceived and manipulated. By transforming static conclusions into dynamic hypotheses, we enable research workflows impossible in traditional systems.

This plan provides a concrete path from concept to working prototype in just 4 weeks, focusing on demonstrating the revolutionary potential rather than building complete infrastructure.

The key insight: **Liberation is not just a technical transformation, but a conceptual revolution in genealogical research.**

---

*End of Data Liberation Initiative v1.0*