//! Universal uncertainty expression
//!
//! Supports multiple models of uncertainty: quantum superposition, fuzzy logic,
//! Bayesian probability, narrative explanations, and logical expressions.

use serde::{Deserialize, Serialize};

use crate::Value;

/// Universal certainty/uncertainty expression
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Certainty {
    /// Quantum superposition - multiple states with probabilities
    Quantum(Vec<(String, f64)>),

    /// Fuzzy logic expression
    Fuzzy {
        membership: f64,   // 0.0 to 1.0
        confidence: f64,   // confidence in the membership value
    },

    /// Bayesian probability network
    Bayesian {
        prior: f64,
        likelihood: f64,
        posterior: f64,
    },

    /// Narrative explanation
    Narrative(String),

    /// Logical expression
    Logical(LogicalExpression),

    /// Explicitly unknown
    Unknown,

    /// Composite certainty (combination of multiple certainty models)
    Composite(Vec<Certainty>),

    /// Simple probability (0.0 to 1.0)
    Probability(f64),
}

impl Certainty {
    /// Create a quantum superposition
    pub fn quantum(states: Vec<(impl Into<String>, f64)>) -> Self {
        Self::Quantum(states.into_iter().map(|(s, p)| (s.into(), p)).collect())
    }

    /// Create a fuzzy certainty
    pub fn fuzzy(membership: f64, confidence: f64) -> Self {
        Self::Fuzzy {
            membership,
            confidence,
        }
    }

    /// Create a Bayesian certainty
    pub fn bayesian(prior: f64, likelihood: f64, posterior: f64) -> Self {
        Self::Bayesian {
            prior,
            likelihood,
            posterior,
        }
    }

    /// Create a narrative certainty
    pub fn narrative(explanation: impl Into<String>) -> Self {
        Self::Narrative(explanation.into())
    }

    /// Create a simple probability
    pub fn probability(p: f64) -> Self {
        Self::Probability(p.clamp(0.0, 1.0))
    }

    /// Create an unknown certainty
    pub fn unknown() -> Self {
        Self::Unknown
    }

    /// Get the most likely state from a quantum certainty
    pub fn most_likely(&self) -> Option<(&str, f64)> {
        match self {
            Self::Quantum(ref states) => states
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .map(|(s, p)| (s.as_str(), *p)),
            Self::Probability(p) => Some(("true", *p)),
            Self::Fuzzy { membership, .. } => Some(("fuzzy", *membership)),
            Self::Bayesian { posterior, .. } => Some(("bayesian", *posterior)),
            _ => None,
        }
    }

    /// Check if this is certain (probability > threshold)
    pub fn is_certain(&self, threshold: f64) -> bool {
        match self {
            Self::Probability(p) => *p >= threshold,
            Self::Fuzzy { membership, .. } => *membership >= threshold,
            Self::Bayesian { posterior, .. } => *posterior >= threshold,
            Self::Quantum(_states) => {
                if let Some((_, p)) = self.most_likely() {
                    p >= threshold
                } else {
                    false
                }
            }
            Self::Unknown => false,
            _ => false,
        }
    }
}

/// Logical expression for complex certainty
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "op", content = "args")]
pub enum LogicalExpression {
    /// Logical AND
    And(Vec<LogicalExpression>),

    /// Logical OR
    Or(Vec<LogicalExpression>),

    /// Logical NOT
    Not(Box<LogicalExpression>),

    /// Logical IMPLIES
    Implies(Box<LogicalExpression>, Box<LogicalExpression>),

    /// Predicate with arguments
    Predicate(String, Vec<Value>),
}

impl LogicalExpression {
    /// Create an AND expression
    pub fn and(expressions: Vec<LogicalExpression>) -> Self {
        Self::And(expressions)
    }

    /// Create an OR expression
    pub fn or(expressions: Vec<LogicalExpression>) -> Self {
        Self::Or(expressions)
    }

    /// Create a NOT expression
    pub fn not(expression: LogicalExpression) -> Self {
        Self::Not(Box::new(expression))
    }

    /// Create an IMPLIES expression
    pub fn implies(premise: LogicalExpression, conclusion: LogicalExpression) -> Self {
        Self::Implies(Box::new(premise), Box::new(conclusion))
    }

    /// Create a predicate
    pub fn predicate(name: impl Into<String>, args: Vec<Value>) -> Self {
        Self::Predicate(name.into(), args)
    }
}

impl Default for Certainty {
    fn default() -> Self {
        Self::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_certainty() {
        let certainty = Certainty::quantum(vec![
            ("same_person", 0.75),
            ("different_person", 0.20),
            ("insufficient_evidence", 0.05),
        ]);

        match &certainty {
            Certainty::Quantum(states) => {
                assert_eq!(states.len(), 3);
                assert_eq!(states[0].0, "same_person");
                assert_eq!(states[0].1, 0.75);
            }
            _ => panic!("Expected Quantum"),
        }

        let (state, prob) = certainty.most_likely().unwrap();
        assert_eq!(state, "same_person");
        assert_eq!(prob, 0.75);
    }

    #[test]
    fn test_fuzzy_certainty() {
        let certainty = Certainty::fuzzy(0.8, 0.9);
        match certainty {
            Certainty::Fuzzy {
                membership,
                confidence,
            } => {
                assert_eq!(membership, 0.8);
                assert_eq!(confidence, 0.9);
            }
            _ => panic!("Expected Fuzzy"),
        }
    }

    #[test]
    fn test_bayesian_certainty() {
        let certainty = Certainty::bayesian(0.5, 0.8, 0.75);
        match certainty {
            Certainty::Bayesian {
                prior,
                likelihood,
                posterior,
            } => {
                assert_eq!(prior, 0.5);
                assert_eq!(likelihood, 0.8);
                assert_eq!(posterior, 0.75);
            }
            _ => panic!("Expected Bayesian"),
        }
    }

    #[test]
    fn test_narrative_certainty() {
        let certainty = Certainty::narrative("Likely based on location proximity");
        match certainty {
            Certainty::Narrative(s) => {
                assert!(s.contains("location proximity"));
            }
            _ => panic!("Expected Narrative"),
        }
    }

    #[test]
    fn test_probability_certainty() {
        let certainty = Certainty::probability(0.85);
        assert!(certainty.is_certain(0.8));
        assert!(!certainty.is_certain(0.9));
    }

    #[test]
    fn test_logical_expression() {
        let pred1 = LogicalExpression::predicate(
            "same_location",
            vec![Value::Text("Boston".to_string())],
        );
        let pred2 = LogicalExpression::predicate(
            "same_timeframe",
            vec![Value::Integer(1850)],
        );

        let and_expr = LogicalExpression::and(vec![pred1, pred2]);
        assert!(matches!(and_expr, LogicalExpression::And(_)));

        let certainty = Certainty::Logical(and_expr);
        assert!(matches!(certainty, Certainty::Logical(_)));
    }

    #[test]
    fn test_composite_certainty() {
        let certainty = Certainty::Composite(vec![
            Certainty::probability(0.75),
            Certainty::narrative("Based on census records"),
        ]);

        match certainty {
            Certainty::Composite(certs) => {
                assert_eq!(certs.len(), 2);
            }
            _ => panic!("Expected Composite"),
        }
    }

    #[test]
    fn test_unknown_certainty() {
        let certainty = Certainty::unknown();
        assert!(!certainty.is_certain(0.5));
        assert_eq!(certainty.most_likely(), None);
    }

    #[test]
    fn test_probability_clamping() {
        let certainty = Certainty::probability(1.5); // > 1.0
        match certainty {
            Certainty::Probability(p) => assert_eq!(p, 1.0),
            _ => panic!("Expected Probability"),
        }

        let certainty = Certainty::probability(-0.5); // < 0.0
        match certainty {
            Certainty::Probability(p) => assert_eq!(p, 0.0),
            _ => panic!("Expected Probability"),
        }
    }
}
