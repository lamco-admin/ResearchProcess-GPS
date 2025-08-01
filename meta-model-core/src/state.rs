// State machine support for meta-model entities

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use chrono::{DateTime, Utc};

use crate::common::AgentId;

/// Trait for types that can be used as states
pub trait State: Debug + Clone + PartialEq + Send + Sync {
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
pub struct StateTransition<S> {
    pub from: S,
    pub to: S,
    pub triggered_by: AgentId,
    pub timestamp: DateTime<Utc>,
    pub reason: Option<String>,
}

/// Trait for entities that have state machines
pub trait StateMachine: Send + Sync {
    type State: State;
    
    /// Get the current state
    fn current_state(&self) -> &Self::State;
    
    /// Get the state history
    fn state_history(&self) -> Vec<StateTransition<Self::State>>;
    
    /// Check if a transition is valid
    fn can_transition(&self, to: &Self::State) -> bool;
    
    /// Perform a state transition
    fn transition(
        &mut self,
        to: Self::State,
        triggered_by: AgentId,
        reason: Option<String>,
    ) -> Result<(), String>;
    
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
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )*
        }
        
        impl $crate::state::State for $name {
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
    fn validate_transition(&self, from: &S, to: &S) -> Result<(), String>;
    
    /// Get the initial state
    fn initial_state() -> S;
    
    /// Get terminal states
    fn terminal_states() -> Vec<S>;
}

/// Common state transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRule<S> {
    pub from: S,
    pub to: Vec<S>,
    pub condition: Option<String>,
}

/// State machine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineConfig<S> {
    pub initial_state: S,
    pub terminal_states: Vec<S>,
    pub error_states: Vec<S>,
    pub transitions: Vec<TransitionRule<S>>,
}

impl<S: State + PartialEq> StateMachineConfig<S> {
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

// Common state types for meta-model entities

define_states! {
    /// Entity lifecycle states
    pub enum EntityState {
        /// Entity is being drafted/created
        Draft,
        /// Entity is active and usable
        Active,
        /// Entity is archived but accessible
        Archived,
        /// Entity is marked for deletion
        PendingDeletion,
        /// Entity has been deleted
        Deleted,
    }
}

impl EntityState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Deleted)
    }
}

define_states! {
    /// Process lifecycle states
    pub enum ProcessState {
        /// Process is being planned
        Planning,
        /// Process is ready to start
        Ready,
        /// Process is actively running
        Active,
        /// Process is paused
        Paused,
        /// Process completed successfully
        Completed,
        /// Process failed
        Failed,
        /// Process was cancelled
        Cancelled,
    }
}

impl ProcessState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
    
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Failed)
    }
}

define_states! {
    /// Workspace lifecycle states
    pub enum WorkspaceState {
        /// Workspace is being set up
        Initializing,
        /// Workspace is active
        Active,
        /// Workspace is suspended
        Suspended,
        /// Workspace is being archived
        Archiving,
        /// Workspace is archived
        Archived,
    }
}

impl WorkspaceState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Archived)
    }
}

// State machine implementations for core entities

/// Entity state machine config
pub fn entity_state_config() -> StateMachineConfig<EntityState> {
    StateMachineConfig {
        initial_state: EntityState::Draft,
        terminal_states: vec![EntityState::Deleted],
        error_states: vec![],
        transitions: vec![
            TransitionRule {
                from: EntityState::Draft,
                to: vec![EntityState::Active, EntityState::Deleted],
                condition: None,
            },
            TransitionRule {
                from: EntityState::Active,
                to: vec![EntityState::Archived, EntityState::PendingDeletion],
                condition: None,
            },
            TransitionRule {
                from: EntityState::Archived,
                to: vec![EntityState::Active, EntityState::PendingDeletion],
                condition: None,
            },
            TransitionRule {
                from: EntityState::PendingDeletion,
                to: vec![EntityState::Active, EntityState::Deleted],
                condition: None,
            },
        ],
    }
}

/// Process state machine config
pub fn process_state_config() -> StateMachineConfig<ProcessState> {
    StateMachineConfig {
        initial_state: ProcessState::Planning,
        terminal_states: vec![ProcessState::Completed, ProcessState::Failed, ProcessState::Cancelled],
        error_states: vec![ProcessState::Failed],
        transitions: vec![
            TransitionRule {
                from: ProcessState::Planning,
                to: vec![ProcessState::Ready, ProcessState::Cancelled],
                condition: None,
            },
            TransitionRule {
                from: ProcessState::Ready,
                to: vec![ProcessState::Active, ProcessState::Cancelled],
                condition: None,
            },
            TransitionRule {
                from: ProcessState::Active,
                to: vec![ProcessState::Paused, ProcessState::Completed, ProcessState::Failed, ProcessState::Cancelled],
                condition: None,
            },
            TransitionRule {
                from: ProcessState::Paused,
                to: vec![ProcessState::Active, ProcessState::Cancelled],
                condition: None,
            },
        ],
    }
}

/// Workspace state machine config
pub fn workspace_state_config() -> StateMachineConfig<WorkspaceState> {
    StateMachineConfig {
        initial_state: WorkspaceState::Initializing,
        terminal_states: vec![WorkspaceState::Archived],
        error_states: vec![],
        transitions: vec![
            TransitionRule {
                from: WorkspaceState::Initializing,
                to: vec![WorkspaceState::Active],
                condition: None,
            },
            TransitionRule {
                from: WorkspaceState::Active,
                to: vec![WorkspaceState::Suspended, WorkspaceState::Archiving],
                condition: None,
            },
            TransitionRule {
                from: WorkspaceState::Suspended,
                to: vec![WorkspaceState::Active, WorkspaceState::Archiving],
                condition: None,
            },
            TransitionRule {
                from: WorkspaceState::Archiving,
                to: vec![WorkspaceState::Archived],
                condition: None,
            },
        ],
    }
}