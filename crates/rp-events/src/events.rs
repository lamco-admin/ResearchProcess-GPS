use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use rp_core::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub event_id: Uuid,
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_version: i64,
    pub occurred_at: DateTime<Utc>,
    pub actor_id: Uuid,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DomainEvent {
    // Layer 1: Core Genealogical Entities
    Theory(TheoryEvent),
    IdentityPersona(IdentityPersonaEvent),
    Person(PersonEvent),
    Source(SourceEvent),
    Evidence(EvidenceEvent),
    Citation(CitationEvent),
    Confidence(ConfidenceEvent),
    Relationship(RelationshipEvent),
    AnalysisReport(AnalysisReportEvent),
    ProofStatement(ProofStatementEvent),
    Fact(FactEvent),
    
    // Layer 2: Research Process Entities
    Researcher(ResearcherEvent),
    ResearchLog(ResearchLogEvent),
    ResearchSession(ResearchSessionEvent),
    ResearchActivity(ResearchActivityEvent),
    WorkProduct(WorkProductEvent),
    Analysis(AnalysisEvent),
    
    // Layer 3: Infrastructure & Metadata
    Workspace(WorkspaceEvent),
    MethodologyConfig(MethodologyConfigEvent),
    ModuleConfig(ModuleConfigEvent),
    StandardsRegistry(StandardsRegistryEvent),
    TemplateRegistry(TemplateRegistryEvent),
    ValidationRule(ValidationRuleEvent),
}

// Theory Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum TheoryEvent {
    Created {
        question: String,
        hypothesis: String,
        researcher_id: Uuid,
    },
    Updated {
        question: Option<String>,
        hypothesis: Option<String>,
        details: Option<String>,
    },
    StateChanged {
        from_state: String,
        to_state: String,
        reason: String,
    },
    ComplianceConfigured {
        standard: String,
        config: serde_json::Value,
    },
    WorkProductAdded {
        product_type: String,
        product_id: Uuid,
    },
    ContributorAdded {
        contributor_id: Uuid,
    },
    Abandoned {
        reason: String,
    },
}

// IdentityPersona Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum IdentityPersonaEvent {
    Created {
        name: Option<String>,
        researcher_id: Uuid,
    },
    Updated {
        name: Option<String>,
        details: serde_json::Value,
    },
    StateChanged {
        from_state: String,
        to_state: String,
        reason: String,
    },
    EvidenceAdded {
        evidence_id: Uuid,
        evidence_type: String,
    },
    PromotedToPerson {
        person_id: Uuid,
        criteria_met: Vec<String>,
    },
    Merged {
        into_identity_id: Uuid,
        reason: String,
    },
}

// Person Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum PersonEvent {
    Created {
        from_identities: Vec<Uuid>,
        concluded_by: Uuid,
    },
    Updated {
        details: serde_json::Value,
    },
    StateChanged {
        from_state: String,
        to_state: String,
        reason: String,
    },
    RelationshipConfirmed {
        relationship_id: Uuid,
        relationship_type: String,
    },
    DemotedToIdentity {
        identity_id: Uuid,
        reason: String,
    },
    Reviewed {
        reviewer_id: Uuid,
        review_notes: String,
    },
}

// Source Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum SourceEvent {
    Created {
        source_type: String,
        title: String,
        researcher_id: Uuid,
    },
    Updated {
        title: Option<String>,
        details: serde_json::Value,
    },
    QualityAssessed {
        dimensions: HashMap<String, f64>,
        assessor_id: Uuid,
    },
    CitationTemplateSet {
        template_id: String,
        template_fields: HashMap<String, String>,
    },
    ChildSourceAdded {
        child_id: Uuid,
    },
    LocationChanged {
        new_location: String,
    },
}

// Evidence Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum EvidenceEvent {
    Extracted {
        source_id: Uuid,
        original_text: String,
        researcher_id: Uuid,
    },
    Updated {
        interpreted_text: Option<String>,
        details: serde_json::Value,
    },
    TypeChanged {
        from_type: String,
        to_type: String,
        reason: String,
    },
    Verified {
        verifier_id: Uuid,
        verification_notes: String,
    },
    Disproven {
        reason: String,
        disproof_evidence: Vec<Uuid>,
    },
    Tagged {
        tags: Vec<String>,
    },
}

// Citation Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum CitationEvent {
    Created {
        from_entity: Uuid,
        from_type: String,
        to_source: Uuid,
        researcher_id: Uuid,
    },
    Updated {
        details: serde_json::Value,
    },
    StateChanged {
        from_state: String,
        to_state: String,
    },
    ElementAdded {
        element_name: String,
        original: String,
        interpreted: String,
        confidence: f64,
    },
    FormatGenerated {
        style: String,
        formatted_text: String,
    },
}

// Confidence Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum ConfidenceEvent {
    Assessed {
        level: f64,
        methodology: String,
        analysis: String,
        assessor_id: Uuid,
    },
    DimensionScored {
        dimension: String,
        score: f64,
        reasoning: String,
    },
    PeerReviewed {
        reviewer_id: Uuid,
        review_notes: String,
        agreed: bool,
    },
    Updated {
        new_analysis: String,
        changes: serde_json::Value,
    },
}

// Relationship Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum RelationshipEvent {
    Created {
        from_entity: Uuid,
        to_entity: Uuid,
        relationship_type: String,
        researcher_id: Uuid,
    },
    Updated {
        details: serde_json::Value,
    },
    Confirmed {
        evidence_ids: Vec<Uuid>,
        confidence_id: Uuid,
    },
    Disputed {
        disputer_id: Uuid,
        reason: String,
    },
    Dissolved {
        reason: String,
    },
}

// AnalysisReport Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum AnalysisReportEvent {
    Created {
        theory_id: Uuid,
        analyst_id: Uuid,
    },
    EvidenceAdded {
        evidence_id: Uuid,
        weight: f64,
    },
    CorrelationIdentified {
        evidence_ids: Vec<Uuid>,
        correlation_type: String,
        strength: f64,
    },
    ConflictResolved {
        conflicting_ids: Vec<Uuid>,
        resolution: String,
        reasoning: String,
    },
    ConclusionReached {
        conclusion: String,
        supporting_evidence: Vec<Uuid>,
    },
}

// ProofStatement Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum ProofStatementEvent {
    Created {
        theory_id: Uuid,
        author_id: Uuid,
    },
    Updated {
        sections: serde_json::Value,
    },
    EvidencePresented {
        evidence_id: Uuid,
        presentation_order: i32,
    },
    ArgumentStructured {
        argument_type: String,
        premises: Vec<String>,
        conclusion: String,
    },
    Published {
        version: String,
        format: String,
    },
}

// Fact Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum FactEvent {
    Asserted {
        statement: String,
        evidence_ids: Vec<Uuid>,
        asserter_id: Uuid,
    },
    Updated {
        statement: Option<String>,
        details: serde_json::Value,
    },
    Verified {
        verifier_id: Uuid,
        verification_method: String,
    },
    Challenged {
        challenger_id: Uuid,
        challenge_reason: String,
        counter_evidence: Vec<Uuid>,
    },
    Retracted {
        reason: String,
    },
}

// Researcher Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum ResearcherEvent {
    Created {
        name: String,
        researcher_type: String,
    },
    Updated {
        name: Option<String>,
        contact_info: serde_json::Value,
    },
    StateChanged {
        from_state: String,
        to_state: String,
    },
    CredentialAdded {
        credential_type: String,
        credential_number: String,
        issued_date: DateTime<Utc>,
    },
    TeamMemberAdded {
        member_id: Uuid,
        role: String,
    },
    PermissionGranted {
        resource_type: String,
        permission_level: String,
    },
    SpecializationAdded {
        specialization: String,
    },
}

// ResearchLog Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum ResearchLogEvent {
    Created {
        log_type: String,
        researcher_id: Uuid,
    },
    EntryAdded {
        entry_type: String,
        content: serde_json::Value,
        timestamp: DateTime<Utc>,
    },
    SessionRecorded {
        session_id: Uuid,
        duration_minutes: i32,
    },
    DataExtracted {
        source_id: Uuid,
        extracted_data: serde_json::Value,
    },
    Exported {
        format: String,
        export_path: String,
    },
}

// ResearchSession Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum ResearchSessionEvent {
    Started {
        session_type: String,
        researcher_id: Uuid,
    },
    ActivityRecorded {
        activity_id: Uuid,
        activity_type: String,
    },
    DataCaptured {
        capture_source: String,
        data: serde_json::Value,
    },
    Paused {
        reason: Option<String>,
    },
    Resumed,
    Ended {
        duration_minutes: i32,
        outputs_generated: Vec<Uuid>,
    },
}

// ResearchActivity Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum ResearchActivityEvent {
    Created {
        activity_type: String,
        target_entity: Option<Uuid>,
    },
    ParametersSet {
        parameters: HashMap<String, serde_json::Value>,
    },
    ResultRecorded {
        results: serde_json::Value,
        success: bool,
    },
    DurationRecorded {
        duration_seconds: i64,
    },
}

// WorkProduct Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum WorkProductEvent {
    Created {
        product_type: String,
        template_id: String,
        creator_id: Uuid,
    },
    Updated {
        content: serde_json::Value,
    },
    Validated {
        validation_rules: Vec<String>,
        passed: bool,
        issues: Vec<String>,
    },
    Exported {
        format: String,
        destination: String,
    },
    VersionCreated {
        version_number: String,
        changes: Vec<String>,
    },
}

// Analysis Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum AnalysisEvent {
    Started {
        analysis_type: String,
        scope: serde_json::Value,
        analyst_id: Uuid,
    },
    DataProcessed {
        data_count: i32,
        processing_time_ms: i64,
    },
    PatternIdentified {
        pattern_type: String,
        confidence: f64,
        entities_involved: Vec<Uuid>,
    },
    InsightGenerated {
        insight_type: String,
        description: String,
        supporting_data: serde_json::Value,
    },
    Completed {
        duration_minutes: i32,
        findings_count: i32,
    },
}

// Workspace Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum WorkspaceEvent {
    Created {
        name: String,
        owner_id: Uuid,
    },
    MethodologyActivated {
        methodology: String,
        version: String,
    },
    ModuleEnabled {
        module_name: String,
        config: serde_json::Value,
    },
    PreferenceSet {
        preference_key: String,
        preference_value: serde_json::Value,
    },
    TheoryOpened {
        theory_id: Uuid,
    },
    TheoryClosed {
        theory_id: Uuid,
    },
}

// MethodologyConfig Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum MethodologyConfigEvent {
    Created {
        methodology_name: String,
        version: String,
    },
    WorkflowDefined {
        stages: Vec<String>,
        transitions: HashMap<String, Vec<String>>,
    },
    RequirementAdded {
        requirement_name: String,
        specification: serde_json::Value,
    },
    RuleConfigured {
        rule_type: String,
        rule_spec: serde_json::Value,
    },
    FeatureToggled {
        feature_name: String,
        enabled: bool,
    },
}

// ModuleConfig Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum ModuleConfigEvent {
    Created {
        module_type: String,
        module_name: String,
    },
    CapabilityAdded {
        capability: String,
    },
    DependencyDeclared {
        dependency: String,
        version_spec: String,
    },
    SettingConfigured {
        setting_key: String,
        setting_value: serde_json::Value,
    },
    TemplateRegistered {
        template_name: String,
        template_ref: String,
    },
}

// StandardsRegistry Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum StandardsRegistryEvent {
    StandardRegistered {
        standard_name: String,
        version: String,
        config: serde_json::Value,
    },
    StandardActivated {
        standard_name: String,
    },
    StandardDeactivated {
        standard_name: String,
        reason: String,
    },
    CompatibilityDefined {
        standard: String,
        compatible_with: Vec<String>,
    },
    MigrationRuleAdded {
        from_version: String,
        to_version: String,
        migration_spec: serde_json::Value,
    },
}

// TemplateRegistry Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum TemplateRegistryEvent {
    TemplateRegistered {
        template_name: String,
        category: String,
        template_spec: serde_json::Value,
    },
    TemplateCategorized {
        template_name: String,
        categories: Vec<String>,
    },
    CompatibilityMatrixUpdated {
        template: String,
        compatible_with: Vec<String>,
    },
    TemplateDeprecated {
        template_name: String,
        replacement: Option<String>,
    },
}

// ValidationRule Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum ValidationRuleEvent {
    Created {
        rule_name: String,
        rule_type: String,
    },
    SpecificationDefined {
        rule_spec: serde_json::Value,
    },
    SeveritySet {
        severity: String,
    },
    MessageConfigured {
        error_message: String,
        help_text: Option<String>,
    },
    RuleEnabled,
    RuleDisabled {
        reason: String,
    },
}

impl DomainEvent {
    pub fn event_type(&self) -> &str {
        match self {
            DomainEvent::Theory(_) => "Theory",
            DomainEvent::IdentityPersona(_) => "IdentityPersona",
            DomainEvent::Person(_) => "Person",
            DomainEvent::Source(_) => "Source",
            DomainEvent::Evidence(_) => "Evidence",
            DomainEvent::Citation(_) => "Citation",
            DomainEvent::Confidence(_) => "Confidence",
            DomainEvent::Relationship(_) => "Relationship",
            DomainEvent::AnalysisReport(_) => "AnalysisReport",
            DomainEvent::ProofStatement(_) => "ProofStatement",
            DomainEvent::Fact(_) => "Fact",
            DomainEvent::Researcher(_) => "Researcher",
            DomainEvent::ResearchLog(_) => "ResearchLog",
            DomainEvent::ResearchSession(_) => "ResearchSession",
            DomainEvent::ResearchActivity(_) => "ResearchActivity",
            DomainEvent::WorkProduct(_) => "WorkProduct",
            DomainEvent::Analysis(_) => "Analysis",
            DomainEvent::Workspace(_) => "Workspace",
            DomainEvent::MethodologyConfig(_) => "MethodologyConfig",
            DomainEvent::ModuleConfig(_) => "ModuleConfig",
            DomainEvent::StandardsRegistry(_) => "StandardsRegistry",
            DomainEvent::TemplateRegistry(_) => "TemplateRegistry",
            DomainEvent::ValidationRule(_) => "ValidationRule",
        }
    }
}