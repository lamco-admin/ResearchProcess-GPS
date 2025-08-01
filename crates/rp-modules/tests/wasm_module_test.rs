//! Test loading and executing WASM modules

use std::path::PathBuf;
use rp_modules::{
    loader::ModuleLoader,
    capabilities::{ModuleCapabilities, EntityPermission},
    resource_limits::ModuleResourceLimits,
    communication::ModuleChannel,
    context::ModuleContext,
    ModuleMessage,
    module::ModuleStatus,
};
use rp_core::EntityId;
use uuid::Uuid;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_load_wasm_module() {
    // Path to the WASM module
    let wasm_path = PathBuf::from("../../target/wasm32-wasip1/release/research_log_wasm.wasm");
    
    // Skip test if WASM file doesn't exist
    if !wasm_path.exists() {
        eprintln!("WASM module not found at {:?}, skipping test", wasm_path);
        return;
    }
    
    // Create capabilities
    let mut capabilities = ModuleCapabilities::default();
    capabilities.entity_permissions.insert(
        "ResearchLog".to_string(),
        EntityPermission::all(),
    );
    
    // Create resource limits
    let limits = ModuleResourceLimits {
        memory_bytes: 64 * 1024 * 1024, // 64MB
        table_elements: 10000,
        instances: 10,
        cpu_time_ms: 100,
        storage_bytes: 10 * 1024 * 1024, // 10MB
        max_concurrent_ops: 10,
        entity_ops_per_minute: 1000,
        events_per_minute: 1000,
    };
    
    // Create module loader
    let loader = ModuleLoader::new().expect("Failed to create module loader");
    
    // Create module context
    let (host_sender, mut host_receiver) = ModuleChannel::create();
    let context = ModuleContext {
        instance_id: Uuid::new_v4(),
        workspace_id: EntityId::new(),
        actor_id: EntityId::new(),
        capabilities,
        host_channel: host_sender,
    };
    
    // Load the WASM module
    let result = loader.load_wasm_module(wasm_path, context, limits).await;
    assert!(result.is_ok(), "Failed to load WASM module: {:?}", result.err());
    
    let module_id = result.unwrap();
    
    // Verify module is loaded
    assert!(loader.is_loaded(&module_id));
    
    // Module should be in Loaded state initially
    let info = loader.get_module(&module_id);
    assert!(info.is_some());
    assert_eq!(info.unwrap().metadata.status, ModuleStatus::Loaded);
    
    // Initialize the module
    let init_result = loader.initialize_module(&module_id).await;
    assert!(init_result.is_ok(), "Failed to initialize module: {:?}", init_result.err());
    
    // Now module should be running
    let info = loader.get_module(&module_id);
    assert!(info.is_some());
    let info = info.unwrap();
    assert_eq!(info.metadata.status, ModuleStatus::Running);
    
    // Test execute command
    let command_result = loader.execute_command(
        &module_id,
        "create_log",
        serde_json::json!({
            "title": "Test Research Log"
        })
    ).await;
    
    assert!(command_result.is_ok(), "Command execution failed: {:?}", command_result.err());
    
    let response = command_result.unwrap();
    assert!(response.get("log_id").is_some());
    assert_eq!(response.get("status"), Some(&serde_json::json!("created")));
    
    // Test receiving events from module
    let event_timeout = timeout(Duration::from_secs(1), host_receiver.recv()).await;
    if let Ok(Some(message)) = event_timeout {
        match message {
            ModuleMessage::EmitEvent { event } => {
                println!("Received event from module: {:?}", event);
            }
            _ => {}
        }
    }
    
    // Unload module
    let unload_result = loader.unload_module(module_id).await;
    assert!(unload_result.is_ok());
    assert!(!loader.is_loaded(&module_id));
}

#[tokio::test]
async fn test_wasm_module_resource_limits() {
    let wasm_path = PathBuf::from("../../target/wasm32-wasip1/release/research_log_wasm.wasm");
    
    if !wasm_path.exists() {
        eprintln!("WASM module not found, skipping test");
        return;
    }
    
    // Create very restrictive limits
    let limits = ModuleResourceLimits {
        memory_bytes: 1024 * 1024, // Only 1MB
        table_elements: 100,
        instances: 1,
        cpu_time_ms: 10, // Very short CPU time
        storage_bytes: 1024,
        max_concurrent_ops: 1,
        entity_ops_per_minute: 10,
        events_per_minute: 10,
    };
    
    let loader = ModuleLoader::new().expect("Failed to create module loader");
    let (host_sender, _) = ModuleChannel::create();
    let context = ModuleContext {
        instance_id: Uuid::new_v4(),
        workspace_id: EntityId::new(),
        actor_id: EntityId::new(),
        capabilities: Default::default(),
        host_channel: host_sender,
    };
    
    // This should still load but operations might fail due to limits
    let result = loader.load_wasm_module(wasm_path, context, limits).await;
    assert!(result.is_ok(), "Module loading should succeed even with limits");
}

#[tokio::test]
async fn test_wasm_module_isolation() {
    let wasm_path = PathBuf::from("../../target/wasm32-wasip1/release/research_log_wasm.wasm");
    
    if !wasm_path.exists() {
        eprintln!("WASM module not found, skipping test");
        return;
    }
    
    let loader = ModuleLoader::new().expect("Failed to create module loader");
    
    // Load two instances of the same module
    let limits = ModuleResourceLimits::default();
    
    // First instance
    let (sender1, _) = ModuleChannel::create();
    let context1 = ModuleContext {
        instance_id: Uuid::new_v4(),
        workspace_id: EntityId::new(),
        actor_id: EntityId::new(),
        capabilities: Default::default(),
        host_channel: sender1,
    };
    let module1 = loader.load_wasm_module(wasm_path.clone(), context1, limits.clone()).await.unwrap();
    
    // Second instance
    let (sender2, _) = ModuleChannel::create();
    let context2 = ModuleContext {
        instance_id: Uuid::new_v4(),
        workspace_id: EntityId::new(),
        actor_id: EntityId::new(),
        capabilities: Default::default(),
        host_channel: sender2,
    };
    let module2 = loader.load_wasm_module(wasm_path, context2, limits).await.unwrap();
    
    // Both should be loaded
    assert!(loader.is_loaded(&module1));
    assert!(loader.is_loaded(&module2));
    assert_ne!(module1, module2);
    
    // Initialize both modules
    loader.initialize_module(&module1).await.unwrap();
    loader.initialize_module(&module2).await.unwrap();
    
    // Create logs in both
    let result1 = loader.execute_command(
        &module1,
        "create_log",
        serde_json::json!({"title": "Log 1"})
    ).await.unwrap();
    
    let result2 = loader.execute_command(
        &module2,
        "create_log",
        serde_json::json!({"title": "Log 2"})
    ).await.unwrap();
    
    // They should have different log IDs (isolation)
    assert_ne!(result1.get("log_id"), result2.get("log_id"));
    
    // Clean up
    loader.unload_module(module1).await.unwrap();
    loader.unload_module(module2).await.unwrap();
}