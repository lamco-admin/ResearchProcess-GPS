//! Transaction support for storage operations

use crate::{StorageError, StorageResult};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Transaction handle for atomic operations
pub struct Transaction {
    pub id: Uuid,
    pub state: Arc<Mutex<TransactionState>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionState {
    Active,
    Committed,
    RolledBack,
}

impl Transaction {
    /// Create a new transaction
    pub fn new() -> Self {
        Transaction {
            id: Uuid::new_v4(),
            state: Arc::new(Mutex::new(TransactionState::Active)),
        }
    }
    
    /// Get transaction ID
    pub fn id(&self) -> Uuid {
        self.id
    }
    
    /// Check if transaction is active
    pub async fn is_active(&self) -> bool {
        let state = self.state.lock().await;
        *state == TransactionState::Active
    }
    
    /// Commit the transaction
    pub async fn commit(self) -> StorageResult<()> {
        let mut state = self.state.lock().await;
        match *state {
            TransactionState::Active => {
                *state = TransactionState::Committed;
                Ok(())
            }
            TransactionState::Committed => {
                Err(StorageError::Transaction("Transaction already committed".to_string()))
            }
            TransactionState::RolledBack => {
                Err(StorageError::Transaction("Transaction already rolled back".to_string()))
            }
        }
    }
    
    /// Rollback the transaction
    pub async fn rollback(self) -> StorageResult<()> {
        let mut state = self.state.lock().await;
        match *state {
            TransactionState::Active => {
                *state = TransactionState::RolledBack;
                Ok(())
            }
            TransactionState::Committed => {
                Err(StorageError::Transaction("Cannot rollback committed transaction".to_string()))
            }
            TransactionState::RolledBack => {
                Err(StorageError::Transaction("Transaction already rolled back".to_string()))
            }
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}