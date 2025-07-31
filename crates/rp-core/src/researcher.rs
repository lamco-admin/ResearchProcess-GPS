//! Researcher entity - the person performing genealogical research

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ORCID_REGEX: Regex = Regex::new(r"^\d{4}-\d{4}-\d{4}-\d{3}[\dX]$").unwrap();
}

use crate::{
    entity::EntityMetadata,
    EntityId, impl_entity, impl_validatable,
};

/// A researcher who creates and modifies entities in the system
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Researcher {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Full name of the researcher
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    /// Email address (optional but recommended)
    #[validate(email)]
    pub email: Option<String>,
    
    /// ORCID identifier for academic attribution
    #[validate(regex(path = "*ORCID_REGEX"))]
    pub orcid: Option<String>,
    
    /// Professional credentials (e.g., "CG", "AG", "PhD")
    pub credentials: Vec<String>,
    
    /// Affiliated organizations
    pub affiliations: Vec<Affiliation>,
    
    /// Contact information
    pub contact: ContactInfo,
    
    /// Research specialties and areas of expertise
    pub specialties: Vec<String>,
    
    /// Whether this researcher's work should be publicly attributed
    pub public_attribution: bool,
    
    /// Notes about this researcher
    pub notes: Option<String>,
}

/// Affiliation with an organization
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Affiliation {
    /// Organization name
    #[validate(length(min = 1, max = 255))]
    pub organization: String,
    
    /// Role or position
    pub role: Option<String>,
    
    /// Start date of affiliation
    pub start_date: Option<DateTime<Utc>>,
    
    /// End date of affiliation (None if current)
    pub end_date: Option<DateTime<Utc>>,
}

/// Contact information for a researcher
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct ContactInfo {
    /// Website or professional profile
    #[validate(url)]
    pub website: Option<String>,
    
    /// Social media handles
    pub social_media: Vec<SocialMedia>,
    
    /// Physical address (optional)
    pub address: Option<Address>,
    
    /// Phone number (optional)
    pub phone: Option<String>,
}

/// Social media profile
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SocialMedia {
    /// Platform name (e.g., "Twitter", "LinkedIn")
    pub platform: String,
    
    /// Handle or profile URL
    pub handle: String,
}

/// Physical address
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Address {
    pub street: Option<String>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

impl Researcher {
    /// Create a new researcher
    pub fn new(name: impl Into<String>, created_by: EntityId) -> Self {
        Self {
            metadata: EntityMetadata::new(created_by),
            name: name.into(),
            email: None,
            orcid: None,
            credentials: Vec::new(),
            affiliations: Vec::new(),
            contact: ContactInfo::default(),
            specialties: Vec::new(),
            public_attribution: true,
            notes: None,
        }
    }
    
    /// Get display name with credentials
    pub fn display_name(&self) -> String {
        if self.credentials.is_empty() {
            self.name.clone()
        } else {
            format!("{}, {}", self.name, self.credentials.join(", "))
        }
    }
    
    /// Check if this researcher has a specific credential
    pub fn has_credential(&self, credential: &str) -> bool {
        self.credentials.iter().any(|c| c == credential)
    }
    
    /// Check if this researcher is affiliated with an organization
    pub fn is_affiliated_with(&self, organization: &str) -> bool {
        self.affiliations.iter().any(|a| {
            a.organization == organization && a.end_date.is_none()
        })
    }
    
    /// Get current affiliations
    pub fn current_affiliations(&self) -> Vec<&Affiliation> {
        self.affiliations
            .iter()
            .filter(|a| a.end_date.is_none())
            .collect()
    }
}

// Implement Entity trait
impl_entity!(Researcher, "Researcher");

// Implement Validatable trait
impl_validatable!(Researcher);

/// System researcher for automated operations
pub fn system_researcher() -> Researcher {
    let id = EntityId::from_uuid(uuid::Uuid::nil());
    let mut researcher = Researcher::new("System", id);
    researcher.metadata.id = id;
    researcher.metadata.created_by = id;
    researcher.metadata.modified_by = id;
    researcher.notes = Some("Automated system operations".to_string());
    researcher.public_attribution = false;
    researcher
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_researcher_creation() {
        let creator_id = EntityId::new();
        let researcher = Researcher::new("John Doe", creator_id);
        
        assert_eq!(researcher.name, "John Doe");
        assert_eq!(researcher.created_by(), creator_id);
        assert!(researcher.validate().await.is_valid());
    }
    
    #[tokio::test]
    async fn test_researcher_validation() {
        let creator_id = EntityId::new();
        let mut researcher = Researcher::new("", creator_id); // Empty name
        
        let result = researcher.validate().await;
        assert!(!result.is_valid());
        
        researcher.name = "Valid Name".to_string();
        researcher.email = Some("invalid-email".to_string()); // Invalid email
        
        let result = researcher.validate().await;
        assert!(!result.is_valid());
        
        researcher.email = Some("valid@email.com".to_string());
        let result = researcher.validate().await;
        assert!(result.is_valid());
    }
}