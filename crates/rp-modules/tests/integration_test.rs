//! Module system integration tests

use rp_modules::{
    ModuleLoader, ModuleContext, ModuleCapabilities, ModuleMessage,
    communication::ModuleChannel, module::ModuleType,
};
use rp_core::{EntityId, layer3::WorkspaceId};
use tokio::sync::mpsc;
use uuid::Uuid;
use std::path::PathBuf;

#[tokio::test]
async fn test_load_research_log_module() {
    // Create module loader
    let loader = ModuleLoader::new().unwrap();
    
    // Create module context
    let (tx, _rx) = mpsc::channel(100);
    let mut capabilities = ModuleCapabilities::none();
    
    // Grant capabilities for research log module
    capabilities.entity_read.insert("ResearchLog".to_string());
    capabilities.entity_write.insert("ResearchLog".to_string());
    capabilities.entity_create.insert("ResearchLog".to_string());
    capabilities.event_emit.insert("ResearchLog.*".to_string());
    
    let context = ModuleContext::new(
        Uuid::new_v4(),
        WorkspaceId::new(),
        EntityId::new(),
        capabilities,
        tx,
    );
    
    // Build module path
    let module_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("modules/research-log");
    
    // Load the module
    let module_id = loader.load_module(&module_path, context).await;
    
    // The module should load successfully now
    let module_id = module_id.expect("Failed to load module");
    
    // Verify the module is loaded
    assert!(loader.is_loaded(&module_id));
    
    // Get module info
    let info = loader.get_module(&module_id);
    assert!(info.is_some());
    let info = info.unwrap();
    assert_eq!(info.metadata.module_type, ModuleType::Native);
    
    // In a complete implementation, we would:
    // 1. Successfully load the module
    // 2. Initialize it
    // 3. Send commands and receive responses
    // 4. Test the module lifecycle
}

#[tokio::test]
async fn test_module_communication_protocol() {
    let (mut host_channel, mut module_channel) = ModuleChannel::new(100);
    
    // Simulate sending a command from host to module
    let command_msg = ModuleMessage::CommandRequest {
        id: Uuid::new_v4(),
        command: "create_log".to_string(),
        args: serde_json::json!({
            "title": "Test Research Log"
        }),
    };
    
    host_channel.to_module.send(command_msg.clone()).await.unwrap();
    
    // Module side receives the command
    if let Some(msg) = module_channel.from_module.recv().await {
        match msg {
            ModuleMessage::CommandRequest { command, .. } => {
                assert_eq!(command, "create_log");
                
                // Simulate module sending response
                let response = ModuleMessage::CommandResponse {
                    id: Uuid::new_v4(),
                    result: Ok(serde_json::json!({
                        "log_id": Uuid::new_v4(),
                        "status": "created"
                    })),
                };
                
                module_channel.to_module.send(response).await.unwrap();
            }
            _ => panic!("Expected CommandRequest"),
        }
    }
    
    // Host receives the response
    if let Some(ModuleMessage::CommandResponse { result, .. }) = host_channel.from_module.recv().await {
        assert!(result.is_ok());
    } else {
        panic!("Expected CommandResponse");
    }
}