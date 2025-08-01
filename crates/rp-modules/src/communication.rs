//! Module communication protocol

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use tokio::sync::mpsc;

use rp_storage::StorageEntity;
use rp_events::DomainEvent;

/// Messages between modules and host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleMessage {
    // Host -> Module messages
    
    /// Event notification
    Event(DomainEvent),
    
    /// Command execution request
    CommandRequest { 
        id: Uuid, 
        command: String, 
        args: Value 
    },
    
    /// Query request
    QueryRequest { 
        id: Uuid, 
        query: Query 
    },
    
    /// Configuration update
    ConfigUpdate(Value),
    
    /// Shutdown signal
    Shutdown,
    
    // Module -> Host messages
    
    /// Entity operation request
    EntityOperation { 
        op: EntityOperation, 
        entity: StorageEntity 
    },
    
    /// Event emission
    EmitEvent { 
        event: DomainEvent 
    },
    
    /// Query response
    QueryResponse { 
        id: Uuid, 
        result: QueryResult 
    },
    
    /// Command response
    CommandResponse { 
        id: Uuid, 
        result: Result<Value, String> 
    },
    
    /// Log message
    Log { 
        level: LogLevel, 
        message: String 
    },
    
    /// Status update
    StatusUpdate { 
        status: String,
        details: Option<Value>
    },
}

/// Entity operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityOperation {
    Create,
    Read,
    Update,
    Delete,
}

/// Query types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Query {
    /// Get entities by type
    EntitiesByType { 
        entity_type: String,
        limit: Option<usize>,
        offset: Option<usize>,
    },
    
    /// Get entity by ID
    EntityById { 
        id: Uuid 
    },
    
    /// Search entities
    SearchEntities { 
        entity_type: String,
        query: String,
        limit: Option<usize>,
    },
    
    /// Custom query
    Custom { 
        name: String,
        params: Value,
    },
}

/// Query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryResult {
    /// Single entity
    Entity(Option<StorageEntity>),
    
    /// Multiple entities
    Entities(Vec<StorageEntity>),
    
    /// Custom result
    Custom(Value),
    
    /// Error
    Error(String),
}

/// Log levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Module communication channel
pub struct ModuleChannel {
    /// Send messages to module
    pub to_module: mpsc::Sender<ModuleMessage>,
    
    /// Receive messages from module
    pub from_module: mpsc::Receiver<ModuleMessage>,
}

impl ModuleChannel {
    /// Create a new bidirectional channel
    pub fn new(buffer_size: usize) -> (Self, Self) {
        let (tx1, rx1) = mpsc::channel(buffer_size);
        let (tx2, rx2) = mpsc::channel(buffer_size);
        
        let host_side = ModuleChannel {
            to_module: tx1,
            from_module: rx2,
        };
        
        let module_side = ModuleChannel {
            to_module: tx2,
            from_module: rx1,
        };
        
        (host_side, module_side)
    }
    
    /// Create a new channel with default buffer size
    pub fn create() -> (mpsc::Sender<ModuleMessage>, mpsc::Receiver<ModuleMessage>) {
        mpsc::channel(100)
    }
}