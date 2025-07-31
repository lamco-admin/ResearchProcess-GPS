//! ResearchLog entity - Specialized work product for research documentation
//! Part of Layer 2: Research Process Model

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    work_product::{WorkProduct, WorkProductType},
    entity::{Entity, NestableEntity, EntityData},
    EntityId, impl_validatable, Result,
};

/// Type of research log
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogType {
    /// Single research session
    Session,
    /// Entire project log
    Project,
    /// Repository visit log
    RepositoryVisit,
    /// Online search session
    OnlineSearch,
    /// Analysis session
    AnalysisSession,
    /// Custom log type
    Custom,
}

/// Type of log entry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogEntryType {
    /// Search activity
    Search,
    /// Analysis work
    Analysis,
    /// Discovery of new information
    Discovery,
    /// General note
    Note,
    /// Negative result (searched but not found)
    NegativeResult,
    /// Methodology note
    Methodology,
    /// Planning entry
    Planning,
    /// Review entry
    Review,
}

/// Source of captured data
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CaptureSource {
    /// Type of source
    pub source_type: CaptureSourceType,
    
    /// Source identifier
    #[validate(length(min = 1))]
    pub source_id: String,
    
    /// Configuration for this source
    pub config: serde_json::Value,
    
    /// Whether currently active
    pub is_active: bool,
}

/// Types of capture sources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureSourceType {
    /// Browser extension
    BrowserExtension,
    /// API integration
    ApiIntegration,
    /// Desktop application
    DesktopApp,
    /// Mobile app
    MobileApp,
    /// Manual entry
    Manual,
    /// Import from file
    FileImport,
}

/// Research coverage summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResearchCoverage {
    /// Sources searched
    pub sources_searched: Vec<String>,
    
    /// Time periods covered
    pub time_periods: Vec<TimePeriod>,
    
    /// Geographic areas covered
    pub geographic_areas: Vec<String>,
    
    /// Record types examined
    pub record_types: Vec<String>,
    
    /// Total time spent
    pub total_duration: Duration,
    
    /// Number of positive results
    pub positive_results: u32,
    
    /// Number of negative results
    pub negative_results: u32,
    
    /// Coverage completeness (0.0 to 1.0)
    pub completeness: f64,
}

/// Time period covered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriod {
    /// Start year
    pub start_year: i32,
    
    /// End year
    pub end_year: i32,
    
    /// Description
    pub description: Option<String>,
}

/// Individual research log entry
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ResearchLogEntry {
    /// Unique entry ID
    pub id: EntityId,
    
    /// Entry type
    pub entry_type: LogEntryType,
    
    /// When this occurred
    pub timestamp: DateTime<Utc>,
    
    /// Duration of activity
    pub duration: Option<Duration>,
    
    /// What was searched/analyzed
    #[validate(length(min = 1))]
    pub target_description: String,
    
    /// Repository if applicable
    pub repository: Option<String>,
    
    /// Collection within repository
    pub collection: Option<String>,
    
    /// Search parameters used
    pub search_params: serde_json::Value,
    
    /// Results summary
    pub results_summary: String,
    
    /// Number of results found
    pub result_count: Option<u32>,
    
    /// Evidence created from this activity
    pub evidence_created: Vec<EntityId>,
    
    /// Identities discovered or updated
    pub identities_affected: Vec<EntityId>,
    
    /// Next steps identified
    pub next_steps: Vec<String>,
    
    /// Researcher who performed this
    pub researcher_id: EntityId,
    
    /// Activity ID if from a session
    pub activity_id: Option<EntityId>,
    
    /// Session ID if from a session
    pub session_id: Option<EntityId>,
    
    /// Notes
    pub notes: Option<String>,
    
    /// Tags
    pub tags: Vec<String>,
}

/// Research log - specialized work product
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ResearchLog {
    /// Base work product fields
    #[serde(flatten)]
    pub work_product: WorkProduct,
    
    /// Log type
    pub log_type: LogType,
    
    /// Schema configuration reference
    #[validate(length(min = 1))]
    pub schema_config: String,
    
    /// Log entries
    #[validate(nested)]
    pub entries: Vec<ResearchLogEntry>,
    
    /// Sessions that contributed to this log
    pub session_ids: Vec<EntityId>,
    
    /// Coverage summary
    pub coverage_summary: ResearchCoverage,
    
    /// Auto-capture enabled
    pub auto_capture_enabled: bool,
    
    /// Capture sources
    #[validate(nested)]
    pub capture_sources: Vec<CaptureSource>,
    
    /// Primary researcher maintaining the log
    pub primary_researcher: EntityId,
    
    /// Contributing researchers
    pub contributing_researchers: Vec<EntityId>,
    
    /// Last entry timestamp
    pub last_entry_at: Option<DateTime<Utc>>,
    
    /// Entry count by type
    pub entry_counts: serde_json::Value,
}

impl ResearchLog {
    /// Create a new research log
    pub fn new(
        log_type: LogType,
        schema_config: impl Into<String>,
        theory_id: EntityId,
        primary_researcher: EntityId,
    ) -> Self {
        let work_product = WorkProduct::new(
            WorkProductType::ResearchLog,
            "research-log-v1",
            "1.0.0",
            "2025.1",
            theory_id,
            primary_researcher,
        );
        
        Self {
            work_product,
            log_type,
            schema_config: schema_config.into(),
            entries: vec![],
            session_ids: vec![],
            coverage_summary: ResearchCoverage::default(),
            auto_capture_enabled: true,
            capture_sources: vec![],
            primary_researcher,
            contributing_researchers: vec![],
            last_entry_at: None,
            entry_counts: serde_json::json!({}),
        }
    }
    
    /// Add a manual entry
    pub fn add_entry(&mut self, mut entry: ResearchLogEntry) {
        // Ensure entry has ID
        if entry.id == EntityId::default() {
            entry.id = EntityId::new();
        }
        
        // Update coverage
        self.update_coverage(&entry);
        
        // Track contributor (only if not the primary researcher)
        if entry.researcher_id != self.primary_researcher 
            && !self.contributing_researchers.contains(&entry.researcher_id) {
            self.contributing_researchers.push(entry.researcher_id);
        }
        
        // Update entry counts
        self.update_entry_counts(&entry);
        
        // Update last entry timestamp
        self.last_entry_at = Some(entry.timestamp);
        
        // Add the entry
        self.entries.push(entry);
        
        // Update work product metadata
        self.work_product.metadata.update(self.primary_researcher);
    }
    
    /// Add entries from a research session
    pub fn add_session(&mut self, session_id: EntityId, activities: Vec<ResearchLogEntry>) {
        if !self.session_ids.contains(&session_id) {
            self.session_ids.push(session_id);
        }
        
        for activity in activities {
            self.add_entry(activity);
        }
    }
    
    /// Update coverage summary based on new entry
    fn update_coverage(&mut self, entry: &ResearchLogEntry) {
        // Update sources
        if let Some(repo) = &entry.repository {
            if !self.coverage_summary.sources_searched.contains(repo) {
                self.coverage_summary.sources_searched.push(repo.clone());
            }
        }
        
        // Update duration
        if let Some(duration) = entry.duration {
            self.coverage_summary.total_duration = 
                self.coverage_summary.total_duration + duration;
        }
        
        // Update result counts
        match entry.entry_type {
            LogEntryType::Discovery => {
                self.coverage_summary.positive_results += 1;
            }
            LogEntryType::NegativeResult => {
                self.coverage_summary.negative_results += 1;
            }
            _ => {}
        }
        
        // Recalculate completeness (simplified)
        let total_results = self.coverage_summary.positive_results + 
                           self.coverage_summary.negative_results;
        if total_results > 0 {
            self.coverage_summary.completeness = 
                self.coverage_summary.positive_results as f64 / total_results as f64;
        }
    }
    
    /// Update entry counts
    fn update_entry_counts(&mut self, entry: &ResearchLogEntry) {
        if let Some(counts) = self.entry_counts.as_object_mut() {
            let key = format!("{:?}", entry.entry_type);
            let current = counts.get(&key)
                .and_then(|v| v.as_u64())
                .unwrap_or(0); // Entry count starts at 0
            counts.insert(key, serde_json::json!(current + 1));
        }
    }
    
    /// Get entries by type
    pub fn entries_by_type(&self, entry_type: LogEntryType) -> Vec<&ResearchLogEntry> {
        self.entries.iter()
            .filter(|e| e.entry_type == entry_type)
            .collect()
    }
    
    /// Get entries for a specific time period
    pub fn entries_in_period(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&ResearchLogEntry> {
        self.entries.iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }
    
    /// Get total research time
    pub fn total_research_time(&self) -> Duration {
        self.coverage_summary.total_duration
    }
    
    /// Check if auto-capture is active
    pub fn is_capturing(&self) -> bool {
        self.auto_capture_enabled && 
        self.capture_sources.iter().any(|s| s.is_active)
    }
    
    /// Add capture source
    pub fn add_capture_source(&mut self, source: CaptureSource) {
        self.capture_sources.push(source);
        self.work_product.metadata.update(self.primary_researcher);
    }
    
    /// Generate summary statistics
    pub fn summary_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "total_entries": self.entries.len(),
            "entry_counts": self.entry_counts,
            "total_duration_hours": self.coverage_summary.total_duration.num_hours(),
            "sources_searched": self.coverage_summary.sources_searched.len(),
            "positive_results": self.coverage_summary.positive_results,
            "negative_results": self.coverage_summary.negative_results,
            "completeness": self.coverage_summary.completeness,
            "contributors": self.contributing_researchers.len() + 1,
            "sessions": self.session_ids.len(),
        })
    }
}

// Implement Entity trait manually since ResearchLog contains WorkProduct
#[async_trait]
impl Entity for ResearchLog {
    fn id(&self) -> EntityId {
        self.work_product.metadata.id
    }
    
    fn entity_type(&self) -> &'static str {
        "ResearchLog"
    }
    
    fn created_by(&self) -> EntityId {
        self.work_product.metadata.created_by
    }
    
    fn created_at(&self) -> DateTime<Utc> {
        self.work_product.metadata.created_at
    }
    
    fn modified_by(&self) -> EntityId {
        self.work_product.metadata.modified_by
    }
    
    fn modified_at(&self) -> DateTime<Utc> {
        self.work_product.metadata.modified_at
    }
    
    fn is_active(&self) -> bool {
        self.work_product.metadata.is_active
    }
    
    fn as_entity(&self) -> EntityData {
        EntityData {
            id: self.work_product.metadata.id,
            entity_type: self.entity_type().to_string(),
            created_by: self.work_product.metadata.created_by,
            created_at: self.work_product.metadata.created_at,
            modified_by: self.work_product.metadata.modified_by,
            modified_at: self.work_product.metadata.modified_at,
            is_active: self.work_product.metadata.is_active,
            version: Some(self.work_product.metadata.version),
            data: serde_json::to_value(self)
                .expect("ResearchLog serialization should never fail"),
        }
    }
}

// Implement NestableEntity trait for ResearchLog
#[async_trait]
impl NestableEntity for ResearchLog {
    fn children(&self) -> Vec<EntityId> {
        // ResearchLog can have child log entries and sessions
        let mut children = Vec::new();
        
        // Add session IDs
        children.extend_from_slice(&self.session_ids);
        
        // Add evidence created from entries
        for entry in &self.entries {
            children.extend_from_slice(&entry.evidence_created);
        }
        
        children
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        // ResearchLog can contain sessions, evidence, and other logs
        matches!(entity_type, "ResearchSession" | "Evidence" | "ResearchLog")
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        // Add as session if not already present
        if !self.session_ids.contains(&child_id) {
            self.session_ids.push(child_id);
            self.work_product.metadata.update(self.work_product.metadata.modified_by);
        }
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.session_ids.len();
        self.session_ids.retain(|&id| id != child_id);
        
        // Also check if it's evidence in any entry
        let mut removed_from_entries = false;
        for entry in &mut self.entries {
            let entry_initial_len = entry.evidence_created.len();
            entry.evidence_created.retain(|&id| id != child_id);
            if entry.evidence_created.len() < entry_initial_len {
                removed_from_entries = true;
            }
        }
        
        let removed = self.session_ids.len() < initial_len || removed_from_entries;
        if removed {
            self.work_product.metadata.update(self.work_product.metadata.modified_by);
        }
        Ok(removed)
    }
}

impl_validatable!(ResearchLog);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_research_log_creation() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let log = ResearchLog::new(
            LogType::Project,
            "BCG_STANDARD",
            theory_id,
            researcher_id,
        );
        
        assert_eq!(log.log_type, LogType::Project);
        assert_eq!(log.schema_config, "BCG_STANDARD");
        assert_eq!(log.primary_researcher, researcher_id);
        assert!(log.auto_capture_enabled);
        assert_eq!(log.entries.len(), 0);
    }
    
    #[test]
    fn test_add_entry() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let evidence_id = EntityId::new();
        
        let mut log = ResearchLog::new(
            LogType::OnlineSearch,
            "GPS-2025",
            theory_id,
            researcher_id,
        );
        
        let entry = ResearchLogEntry {
            id: EntityId::new(),
            entry_type: LogEntryType::Search,
            timestamp: Utc::now(),
            duration: Some(Duration::minutes(30)),
            target_description: "1850 Census for Smith family".to_string(),
            repository: Some("FamilySearch".to_string()),
            collection: Some("US Federal Census".to_string()),
            search_params: serde_json::json!({
                "surname": "Smith",
                "given_name": "John",
                "year": 1850,
                "location": "Boston"
            }),
            results_summary: "Found 3 potential matches".to_string(),
            result_count: Some(3),
            evidence_created: vec![evidence_id],
            identities_affected: vec![],
            next_steps: vec!["Review each match in detail".to_string()],
            researcher_id,
            activity_id: None,
            session_id: None,
            notes: Some("Focus on Ward 3".to_string()),
            tags: vec!["census".to_string(), "smith-family".to_string()],
        };
        
        log.add_entry(entry);
        
        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.contributing_researchers.len(), 0); // Primary researcher not in contributors
        assert!(log.coverage_summary.sources_searched.contains(&"FamilySearch".to_string()));
        assert_eq!(log.coverage_summary.total_duration, Duration::minutes(30));
    }
    
    #[test]
    fn test_coverage_tracking() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut log = ResearchLog::new(
            LogType::Project,
            "BCG_STANDARD",
            theory_id,
            researcher_id,
        );
        
        // Add discovery entry
        let discovery = ResearchLogEntry {
            id: EntityId::new(),
            entry_type: LogEntryType::Discovery,
            timestamp: Utc::now(),
            duration: Some(Duration::minutes(15)),
            target_description: "Found birth record".to_string(),
            repository: Some("Ancestry".to_string()),
            collection: None,
            search_params: serde_json::json!({}),
            results_summary: "Birth record confirmed".to_string(),
            result_count: Some(1),
            evidence_created: vec![],
            identities_affected: vec![],
            next_steps: vec![],
            researcher_id,
            activity_id: None,
            session_id: None,
            notes: None,
            tags: vec![],
        };
        
        log.add_entry(discovery);
        
        // Add negative result
        let negative = ResearchLogEntry {
            id: EntityId::new(),
            entry_type: LogEntryType::NegativeResult,
            timestamp: Utc::now(),
            duration: Some(Duration::minutes(45)),
            target_description: "No death record found".to_string(),
            repository: Some("FindMyPast".to_string()),
            collection: None,
            search_params: serde_json::json!({}),
            results_summary: "Exhaustive search yielded no results".to_string(),
            result_count: Some(0),
            evidence_created: vec![],
            identities_affected: vec![],
            next_steps: vec!["Try alternate spellings".to_string()],
            researcher_id,
            activity_id: None,
            session_id: None,
            notes: None,
            tags: vec![],
        };
        
        log.add_entry(negative);
        
        assert_eq!(log.coverage_summary.positive_results, 1);
        assert_eq!(log.coverage_summary.negative_results, 1);
        assert_eq!(log.coverage_summary.completeness, 0.5);
        assert_eq!(log.coverage_summary.total_duration, Duration::minutes(60));
        assert_eq!(log.coverage_summary.sources_searched.len(), 2);
    }
    
    #[test]
    fn test_entry_filtering() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut log = ResearchLog::new(
            LogType::Session,
            "GPS-2025",
            theory_id,
            researcher_id,
        );
        
        // Add various types of entries
        for i in 0..5 {
            let entry_type = match i {
                0 | 1 => LogEntryType::Search,
                2 => LogEntryType::Discovery,
                3 => LogEntryType::Analysis,
                _ => LogEntryType::Note,
            };
            
            let entry = ResearchLogEntry {
                id: EntityId::new(),
                entry_type,
                timestamp: Utc::now() - Duration::hours(i),
                duration: None,
                target_description: format!("Entry {}", i),
                repository: None,
                collection: None,
                search_params: serde_json::json!({}),
                results_summary: format!("Summary {}", i),
                result_count: None,
                evidence_created: vec![],
                identities_affected: vec![],
                next_steps: vec![],
                researcher_id,
                activity_id: None,
                session_id: None,
                notes: None,
                tags: vec![],
            };
            
            log.add_entry(entry);
        }
        
        let search_entries = log.entries_by_type(LogEntryType::Search);
        assert_eq!(search_entries.len(), 2);
        
        let discovery_entries = log.entries_by_type(LogEntryType::Discovery);
        assert_eq!(discovery_entries.len(), 1);
    }
    
    #[test]
    fn test_capture_sources() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut log = ResearchLog::new(
            LogType::OnlineSearch,
            "BCG_STANDARD",
            theory_id,
            researcher_id,
        );
        
        let source = CaptureSource {
            source_type: CaptureSourceType::BrowserExtension,
            source_id: "rp-browser-ext-v1".to_string(),
            config: serde_json::json!({
                "auto_capture": true,
                "capture_screenshots": false,
            }),
            is_active: true,
        };
        
        log.add_capture_source(source);
        
        assert_eq!(log.capture_sources.len(), 1);
        assert!(log.is_capturing());
    }
    
    #[test]
    fn test_summary_stats() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let contributor_id = EntityId::new();
        
        let mut log = ResearchLog::new(
            LogType::Project,
            "GPS-2025",
            theory_id,
            researcher_id,
        );
        
        // Add entries from different researchers
        for i in 0..3 {
            let entry = ResearchLogEntry {
                id: EntityId::new(),
                entry_type: LogEntryType::Search,
                timestamp: Utc::now(),
                duration: Some(Duration::minutes(20)),
                target_description: format!("Search {}", i),
                repository: Some(format!("Repo{}", i)),
                collection: None,
                search_params: serde_json::json!({}),
                results_summary: "Results".to_string(),
                result_count: Some(i as u32),
                evidence_created: vec![],
                identities_affected: vec![],
                next_steps: vec![],
                researcher_id: if i == 2 { contributor_id } else { researcher_id },
                activity_id: None,
                session_id: None,
                notes: None,
                tags: vec![],
            };
            
            log.add_entry(entry);
        }
        
        let stats = log.summary_stats();
        
        assert_eq!(stats["total_entries"], 3);
        assert_eq!(stats["sources_searched"], 3);
        assert_eq!(stats["contributors"], 2); // Primary + 1 contributor
        assert_eq!(stats["total_duration_hours"], 1);
    }
}