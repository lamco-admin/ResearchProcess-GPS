//! State machine traits and utilities

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::{EntityId, Error, Result};

/// Trait for types that can be used as states
pub trait State: Debug + Clone + PartialEq + Send + Sync + Serialize + for<'de> Deserialize<'de> {
    /// Get the state name
    fn name(&self) -> &'static str;
    
    /// Check if this is a terminal state
    fn is_terminal(&self) -> bool {
        false
    }
    
    /// Check if this is an error state
    fn is_error(&self) -> bool {
        false
    }
}

/// State transition information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition<S: State> {
    pub from: S,
    pub to: S,
    pub triggered_by: EntityId,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub reason: Option<String>,
}

/// Trait for entities that have state machines
#[async_trait]
pub trait StateMachine: Send + Sync {
    type State: State;
    
    /// Get the current state
    fn current_state(&self) -> &Self::State;
    
    /// Get the state history
    fn state_history(&self) -> Vec<StateTransition<Self::State>>;
    
    /// Check if a transition is valid
    fn can_transition(&self, to: &Self::State) -> bool;
    
    /// Perform a state transition
    async fn transition(
        &mut self,
        to: Self::State,
        triggered_by: EntityId,
        reason: Option<String>,
    ) -> Result<()>;
    
    /// Get all valid transitions from the current state
    fn valid_transitions(&self) -> Vec<Self::State>;
}

/// Helper macro to define state enums
#[macro_export]
macro_rules! define_states {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[derive(strum_macros::Display, strum_macros::EnumString)]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )*
        }
        
        impl State for $name {
            fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => stringify!($variant),
                    )*
                }
            }
        }
    };
}

/// State machine validation rules
pub trait StateValidation<S: State> {
    /// Validate that a state transition is allowed
    fn validate_transition(&self, from: &S, to: &S) -> Result<()>;
    
    /// Get the initial state
    fn initial_state() -> S;
    
    /// Get terminal states
    fn terminal_states() -> Vec<S>;
}

/// Common state transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRule<S: State> {
    pub from: S,
    pub to: Vec<S>,
    pub condition: Option<String>,
}

/// State machine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineConfig<S: State> {
    pub initial_state: S,
    pub terminal_states: Vec<S>,
    pub error_states: Vec<S>,
    pub transitions: Vec<TransitionRule<S>>,
}

impl<S: State> StateMachineConfig<S> {
    /// Check if a transition is valid according to the rules
    pub fn is_valid_transition(&self, from: &S, to: &S) -> bool {
        self.transitions.iter().any(|rule| {
            &rule.from == from && rule.to.contains(to)
        })
    }
    
    /// Get valid transitions from a state
    pub fn valid_transitions_from(&self, state: &S) -> Vec<S> {
        self.transitions
            .iter()
            .filter(|rule| &rule.from == state)
            .flat_map(|rule| rule.to.clone())
            .collect()
    }
}