//! ResearchActivity entity - Atomic research activities within sessions
//! Part of Layer 2: Research Process Model

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entity::EntityMetadata,
    EntityId, impl_entity, impl_validatable,
};

/// Type of research activity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityType {
    /// Searching for records
    Search,
    /// Extracting information from sources
    Extract,
    /// Analyzing evidence
    Analyze,
    /// Correlating multiple pieces of evidence
    Correlate,
    /// Documenting findings
    Document,
    /// Reviewing existing work
    Review,
    /// Transcribing records
    Transcribe,
    /// Translating records
    Translate,
    /// Planning research strategy
    Plan,
    /// Organizing materials
    Organize,
    /// Verifying information
    Verify,
    /// Custom activity type
    Custom,
}

/// Result quality assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResultQuality {
    /// High quality, reliable results
    High,
    /// Medium quality, some uncertainty
    Medium,
    /// Low quality, significant issues
    Low,
    /// Quality not assessed
    Unknown,
}

/// Tools used in the activity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ToolUsage {
    /// Tool name or identifier
    #[validate(length(min = 1))]
    pub tool_name: String,
    
    /// Tool version if applicable
    pub version: Option<String>,
    
    /// How the tool was used
    pub usage_type: String,
    
    /// Tool-specific parameters
    pub parameters: serde_json::Value,
}

/// Activity result details
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ActivityResult {
    /// Whether the activity succeeded
    pub success: bool,
    
    /// Number of items found/processed
    pub count: Option<u32>,
    
    /// Quality assessment
    pub quality: ResultQuality,
    
    /// Confidence score (0.0 to 1.0)
    #[validate(range(min = 0.0, max = 1.0))]
    pub confidence: Option<f64>,
    
    /// Result details
    pub details: serde_json::Value,
    
    /// Time to first result
    pub time_to_first_result: Option<Duration>,
}

/// Research activity entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ResearchActivity {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Type of activity
    pub activity_type: ActivityType,
    
    /// When activity occurred
    pub timestamp: DateTime<Utc>,
    
    /// Duration of activity
    pub duration: Option<Duration>,
    
    /// Description of the activity
    #[validate(length(min = 1))]
    pub description: String,
    
    /// Target entity being researched
    pub target_entity: Option<EntityId>,
    
    /// Type of target entity
    pub target_type: Option<String>,
    
    /// Search/activity parameters
    pub parameters: serde_json::Value,
    
    /// Tools used
    #[validate(nested)]
    pub tools_used: Vec<ToolUsage>,
    
    /// Activity results
    pub result: ActivityResult,
    
    /// Entities created by this activity
    pub created_entities: Vec<EntityId>,
    
    /// Entities modified by this activity
    pub modified_entities: Vec<EntityId>,
    
    /// Errors encountered
    pub errors: Vec<String>,
    
    /// Warnings generated
    pub warnings: Vec<String>,
    
    /// Researcher who performed the activity
    pub performed_by: EntityId,
    
    /// Session this activity belongs to
    pub session_id: Option<EntityId>,
    
    /// Previous activity in sequence
    pub previous_activity: Option<EntityId>,
    
    /// Next activity in sequence
    pub next_activity: Option<EntityId>,
    
    /// Notes about the activity
    pub notes: Option<String>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Custom fields
    pub custom_fields: serde_json::Value,
}

impl ResearchActivity {
    /// Create a new research activity
    pub fn new(
        activity_type: ActivityType,
        description: impl Into<String>,
        performed_by: EntityId,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            metadata: EntityMetadata {
                id: EntityId::new(),
                created_by: performed_by,
                created_at: now,
                modified_by: performed_by,
                modified_at: now,
                is_active: true,
                version: 1,
                parent_version: None,
            },
            activity_type,
            timestamp: now,
            duration: None,
            description: description.into(),
            target_entity: None,
            target_type: None,
            parameters: serde_json::Value::Object(serde_json::Map::new()),
            tools_used: vec![],
            result: ActivityResult {
                success: true,
                count: None,
                quality: ResultQuality::Unknown,
                confidence: None,
                details: serde_json::Value::Object(serde_json::Map::new()),
                time_to_first_result: None,
            },
            created_entities: vec![],
            modified_entities: vec![],
            errors: vec![],
            warnings: vec![],
            performed_by,
            session_id: None,
            previous_activity: None,
            next_activity: None,
            notes: None,
            tags: vec![],
            custom_fields: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
    
    /// Create a search activity
    pub fn search(
        description: impl Into<String>,
        parameters: serde_json::Value,
        performed_by: EntityId,
    ) -> Self {
        let mut activity = Self::new(ActivityType::Search, description, performed_by);
        activity.parameters = parameters;
        activity
    }
    
    /// Create an extraction activity
    pub fn extract(
        description: impl Into<String>,
        target_entity: EntityId,
        performed_by: EntityId,
    ) -> Self {
        let mut activity = Self::new(ActivityType::Extract, description, performed_by);
        activity.target_entity = Some(target_entity);
        activity
    }
    
    /// Set the target entity
    pub fn with_target(mut self, entity_id: EntityId, entity_type: impl Into<String>) -> Self {
        self.target_entity = Some(entity_id);
        self.target_type = Some(entity_type.into());
        self
    }
    
    /// Set the session ID
    pub fn in_session(mut self, session_id: EntityId) -> Self {
        self.session_id = Some(session_id);
        self
    }
    
    /// Add a tool used
    pub fn add_tool(&mut self, tool: ToolUsage) {
        self.tools_used.push(tool);
        self.metadata.update(self.performed_by);
    }
    
    /// Record success with results
    pub fn record_success(&mut self, count: Option<u32>, quality: ResultQuality, details: serde_json::Value) {
        self.result.success = true;
        self.result.count = count;
        self.result.quality = quality;
        self.result.details = details;
        self.metadata.update(self.performed_by);
    }
    
    /// Record failure
    pub fn record_failure(&mut self, error: impl Into<String>) {
        self.result.success = false;
        self.errors.push(error.into());
        self.metadata.update(self.performed_by);
    }
    
    /// Add created entity
    pub fn add_created_entity(&mut self, entity_id: EntityId) {
        if !self.created_entities.contains(&entity_id) {
            self.created_entities.push(entity_id);
            self.metadata.update(self.performed_by);
        }
    }
    
    /// Add modified entity
    pub fn add_modified_entity(&mut self, entity_id: EntityId) {
        if !self.modified_entities.contains(&entity_id) {
            self.modified_entities.push(entity_id);
            self.metadata.update(self.performed_by);
        }
    }
    
    /// Add warning
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
        self.metadata.update(self.performed_by);
    }
    
    /// Set activity duration
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = Some(duration);
        self.metadata.update(self.performed_by);
    }
    
    /// Complete the activity with timing
    pub fn complete(&mut self) {
        let duration = Utc::now() - self.timestamp;
        self.duration = Some(duration);
        self.metadata.update(self.performed_by);
    }
    
    /// Link to previous activity
    pub fn link_previous(&mut self, previous_id: EntityId) {
        self.previous_activity = Some(previous_id);
        self.metadata.update(self.performed_by);
    }
    
    /// Link to next activity
    pub fn link_next(&mut self, next_id: EntityId) {
        self.next_activity = Some(next_id);
        self.metadata.update(self.performed_by);
    }
    
    /// Check if activity was successful
    pub fn is_successful(&self) -> bool {
        self.result.success && self.errors.is_empty()
    }
    
    /// Get total entities affected
    pub fn entities_affected(&self) -> usize {
        self.created_entities.len() + self.modified_entities.len()
    }
    
    /// Generate activity summary
    pub fn summary(&self) -> serde_json::Value {
        serde_json::json!({
            "type": format!("{:?}", self.activity_type),
            "description": self.description,
            "success": self.result.success,
            "duration_seconds": self.duration.map(|d| d.num_seconds()),
            "entities_created": self.created_entities.len(),
            "entities_modified": self.modified_entities.len(),
            "errors": self.errors.len(),
            "warnings": self.warnings.len(),
            "result_count": self.result.count,
            "quality": format!("{:?}", self.result.quality),
        })
    }
}

impl_entity!(ResearchActivity, "ResearchActivity");
impl_validatable!(ResearchActivity);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_activity_creation() {
        let researcher_id = EntityId::new();
        
        let activity = ResearchActivity::new(
            ActivityType::Search,
            "Search for birth records",
            researcher_id,
        );
        
        assert_eq!(activity.activity_type, ActivityType::Search);
        assert_eq!(activity.description, "Search for birth records");
        assert_eq!(activity.performed_by, researcher_id);
        assert!(activity.result.success);
    }
    
    #[test]
    fn test_search_activity() {
        let researcher_id = EntityId::new();
        
        let params = serde_json::json!({
            "surname": "Smith",
            "given_name": "John",
            "year_range": "1840-1860",
            "location": "Boston"
        });
        
        let activity = ResearchActivity::search(
            "Search for John Smith in Boston 1840-1860",
            params.clone(),
            researcher_id,
        );
        
        assert_eq!(activity.activity_type, ActivityType::Search);
        assert_eq!(activity.parameters, params);
    }
    
    #[test]
    fn test_extract_activity() {
        let researcher_id = EntityId::new();
        let source_id = EntityId::new();
        
        let activity = ResearchActivity::extract(
            "Extract data from 1850 census",
            source_id,
            researcher_id,
        );
        
        assert_eq!(activity.activity_type, ActivityType::Extract);
        assert_eq!(activity.target_entity, Some(source_id));
    }
    
    #[test]
    fn test_with_target() {
        let researcher_id = EntityId::new();
        let target_id = EntityId::new();
        
        let activity = ResearchActivity::new(
            ActivityType::Analyze,
            "Analyze evidence",
            researcher_id,
        )
        .with_target(target_id, "Evidence");
        
        assert_eq!(activity.target_entity, Some(target_id));
        assert_eq!(activity.target_type, Some("Evidence".to_string()));
    }
    
    #[test]
    fn test_tool_usage() {
        let researcher_id = EntityId::new();
        
        let mut activity = ResearchActivity::new(
            ActivityType::Search,
            "Online search",
            researcher_id,
        );
        
        let tool = ToolUsage {
            tool_name: "FamilySearch".to_string(),
            version: Some("3.0".to_string()),
            usage_type: "API".to_string(),
            parameters: serde_json::json!({
                "collection": "US-Census-1850",
                "api_key": "***"
            }),
        };
        
        activity.add_tool(tool);
        
        assert_eq!(activity.tools_used.len(), 1);
        assert_eq!(activity.tools_used[0].tool_name, "FamilySearch");
    }
    
    #[test]
    fn test_record_results() {
        let researcher_id = EntityId::new();
        
        let mut activity = ResearchActivity::new(
            ActivityType::Search,
            "Search census",
            researcher_id,
        );
        
        let details = serde_json::json!({
            "database": "1850-US-Census",
            "search_time_ms": 250,
            "relevance_threshold": 0.8
        });
        
        activity.record_success(Some(5), ResultQuality::High, details.clone());
        
        assert!(activity.result.success);
        assert_eq!(activity.result.count, Some(5));
        assert_eq!(activity.result.quality, ResultQuality::High);
        assert_eq!(activity.result.details, details);
    }
    
    #[test]
    fn test_record_failure() {
        let researcher_id = EntityId::new();
        
        let mut activity = ResearchActivity::new(
            ActivityType::Search,
            "Failed search",
            researcher_id,
        );
        
        activity.record_failure("Connection timeout");
        
        assert!(!activity.result.success);
        assert_eq!(activity.errors.len(), 1);
        assert_eq!(activity.errors[0], "Connection timeout");
        assert!(!activity.is_successful());
    }
    
    #[test]
    fn test_entity_tracking() {
        let researcher_id = EntityId::new();
        let entity1 = EntityId::new();
        let entity2 = EntityId::new();
        let entity3 = EntityId::new();
        
        let mut activity = ResearchActivity::new(
            ActivityType::Extract,
            "Extract records",
            researcher_id,
        );
        
        activity.add_created_entity(entity1);
        activity.add_created_entity(entity2);
        activity.add_modified_entity(entity3);
        activity.add_created_entity(entity1); // Duplicate
        
        assert_eq!(activity.created_entities.len(), 2);
        assert_eq!(activity.modified_entities.len(), 1);
        assert_eq!(activity.entities_affected(), 3);
    }
    
    #[test]
    fn test_warnings() {
        let researcher_id = EntityId::new();
        
        let mut activity = ResearchActivity::new(
            ActivityType::Transcribe,
            "Transcribe document",
            researcher_id,
        );
        
        activity.add_warning("Text partially illegible");
        activity.add_warning("Date format unclear");
        
        assert_eq!(activity.warnings.len(), 2);
        assert!(activity.is_successful()); // Warnings don't make it unsuccessful
    }
    
    #[test]
    fn test_activity_linking() {
        let researcher_id = EntityId::new();
        let prev_id = EntityId::new();
        let next_id = EntityId::new();
        
        let mut activity = ResearchActivity::new(
            ActivityType::Analyze,
            "Middle activity",
            researcher_id,
        );
        
        activity.link_previous(prev_id);
        activity.link_next(next_id);
        
        assert_eq!(activity.previous_activity, Some(prev_id));
        assert_eq!(activity.next_activity, Some(next_id));
    }
    
    #[test]
    fn test_activity_timing() {
        let researcher_id = EntityId::new();
        
        let mut activity = ResearchActivity::new(
            ActivityType::Review,
            "Review documents",
            researcher_id,
        );
        
        // Simulate activity taking time
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        activity.complete();
        
        assert!(activity.duration.is_some());
        assert!(activity.duration.unwrap().num_milliseconds() >= 10);
    }
    
    #[test]
    fn test_activity_summary() {
        let researcher_id = EntityId::new();
        
        let mut activity = ResearchActivity::new(
            ActivityType::Search,
            "Search test",
            researcher_id,
        );
        
        activity.record_success(Some(3), ResultQuality::Medium, serde_json::json!({}));
        activity.add_created_entity(EntityId::new());
        activity.add_warning("Minor issue");
        activity.set_duration(Duration::seconds(30));
        
        let summary = activity.summary();
        
        assert_eq!(summary["type"], "Search");
        assert_eq!(summary["success"], true);
        assert_eq!(summary["duration_seconds"], 30);
        assert_eq!(summary["entities_created"], 1);
        assert_eq!(summary["warnings"], 1);
        assert_eq!(summary["result_count"], 3);
    }
}