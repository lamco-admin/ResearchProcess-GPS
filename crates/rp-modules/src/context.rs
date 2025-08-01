//! Module execution context

use uuid::Uuid;
use tokio::sync::mpsc;

use rp_core::{EntityId, layer3::WorkspaceId};
use crate::{ModuleCapabilities, ModuleMessage};

/// Context provided to modules for execution
#[derive(Clone)]
pub struct ModuleContext {
    /// Module's unique instance ID
    pub instance_id: Uuid,
    
    /// Workspace context
    pub workspace_id: WorkspaceId,
    
    /// Actor (researcher) context
    pub actor_id: EntityId,
    
    /// Granted capabilities
    pub capabilities: ModuleCapabilities,
    
    /// Channel for sending messages to host
    pub host_channel: mpsc::Sender<ModuleMessage>,
}

impl ModuleContext {
    /// Create a new module context
    pub fn new(
        instance_id: Uuid,
        workspace_id: WorkspaceId,
        actor_id: EntityId,
        capabilities: ModuleCapabilities,
        host_channel: mpsc::Sender<ModuleMessage>,
    ) -> Self {
        Self {
            instance_id,
            workspace_id,
            actor_id,
            capabilities,
            host_channel,
        }
    }
    
    /// Send a message to the host
    pub async fn send_to_host(&self, message: ModuleMessage) -> Result<(), mpsc::error::SendError<ModuleMessage>> {
        self.host_channel.send(message).await
    }
}