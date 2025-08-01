// Certainty - Expresses confidence/probability in various ways

use serde::{Serialize, Deserialize};

/// Different ways to express certainty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Certainty {
    /// Simple percentage (0-100)
    Percentage(f64),
    
    /// Enumerated levels
    Level(CertaintyLevel),
    
    /// Fuzzy logic membership
    Fuzzy {
        membership: f64,    // 0.0 to 1.0
        confidence: f64,    // confidence in the membership value
    },
    
    /// Quantum superposition of states
    Quantum(Vec<(String, f64)>), // state -> probability
    
    /// Bayesian probability with priors
    Bayesian {
        prior: f64,
        likelihood: f64,
        posterior: f64,
    },
    
    /// Evidence-based certainty
    Evidence {
        supporting_count: usize,
        contradicting_count: usize,
        quality_score: f64,
    },
    
    /// Narrative/descriptive certainty
    Narrative(String),
    
    /// Unknown/unspecified
    Unknown,
}

/// Predefined certainty levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertaintyLevel {
    Certain,
    VeryLikely,
    Likely,
    Possible,
    Unlikely,
    VeryUnlikely,
    Impossible,
}

impl Certainty {
    /// Create from a simple probability (0.0 to 1.0)
    pub fn from_probability(p: f64) -> Self {
        Certainty::Percentage(p * 100.0)
    }
    
    /// Convert to approximate probability
    pub fn to_probability(&self) -> Option<f64> {
        match self {
            Certainty::Percentage(p) => Some(p / 100.0),
            Certainty::Level(level) => Some(match level {
                CertaintyLevel::Certain => 1.0,
                CertaintyLevel::VeryLikely => 0.9,
                CertaintyLevel::Likely => 0.75,
                CertaintyLevel::Possible => 0.5,
                CertaintyLevel::Unlikely => 0.25,
                CertaintyLevel::VeryUnlikely => 0.1,
                CertaintyLevel::Impossible => 0.0,
            }),
            Certainty::Fuzzy { membership, .. } => Some(*membership),
            Certainty::Bayesian { posterior, .. } => Some(*posterior),
            Certainty::Evidence { supporting_count, contradicting_count, .. } => {
                if supporting_count + contradicting_count > 0 {
                    Some(*supporting_count as f64 / (*supporting_count + *contradicting_count) as f64)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    /// Check if certainty is above a threshold
    pub fn is_above(&self, threshold: f64) -> bool {
        self.to_probability()
            .map(|p| p > threshold)
            .unwrap_or(false)
    }
    
    /// Combine two certainties (various strategies)
    pub fn combine(&self, other: &Certainty) -> Certainty {
        match (self, other) {
            // Combine percentages by averaging
            (Certainty::Percentage(a), Certainty::Percentage(b)) => {
                Certainty::Percentage((a + b) / 2.0)
            }
            
            // Combine quantum states
            (Certainty::Quantum(states_a), Certainty::Quantum(states_b)) => {
                // This is simplified - real quantum combination would be more complex
                let mut combined = states_a.clone();
                for (state, prob) in states_b {
                    if let Some(existing) = combined.iter_mut().find(|(s, _)| s == state) {
                        existing.1 = (existing.1 + prob) / 2.0;
                    } else {
                        combined.push((state.clone(), *prob));
                    }
                }
                // Normalize probabilities
                let sum: f64 = combined.iter().map(|(_, p)| p).sum();
                if sum > 0.0 {
                    for (_, prob) in &mut combined {
                        *prob /= sum;
                    }
                }
                Certainty::Quantum(combined)
            }
            
            // For different types, try to convert to probability
            _ => {
                if let (Some(p1), Some(p2)) = (self.to_probability(), other.to_probability()) {
                    Certainty::Percentage(((p1 + p2) / 2.0) * 100.0)
                } else {
                    Certainty::Unknown
                }
            }
        }
    }
}

impl Default for Certainty {
    fn default() -> Self {
        Certainty::Unknown
    }
}