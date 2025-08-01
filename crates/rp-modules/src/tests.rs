//! Module system tests

#[cfg(test)]
mod loader_tests {
    use crate::{ModuleLoader, ModuleContext, ModuleCapabilities};
    use rp_core::{EntityId, layer3::WorkspaceId};
    use tokio::sync::mpsc;
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_module_loader_creation() {
        let loader = ModuleLoader::new().unwrap();
        assert!(loader.get_instance(&Uuid::new_v4()).is_none());
    }
    
    #[tokio::test]
    async fn test_module_loading_nonexistent() {
        let loader = ModuleLoader::new().unwrap();
        let (tx, _rx) = mpsc::channel(100);
        
        let context = ModuleContext::new(
            Uuid::new_v4(),
            WorkspaceId::new(),
            EntityId::new(),
            ModuleCapabilities::none(),
            tx,
        );
        
        let result = loader.load_module("/nonexistent/path", context).await;
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod capability_tests {
    use crate::capabilities::ModuleCapabilities;
    
    #[test]
    fn test_capability_checking() {
        let caps = ModuleCapabilities::none();
        
        // Test that no capabilities returns false
        assert!(!caps.can_read_entity("ResearchLog"));
        assert!(!caps.can_write_entity("ResearchLog"));
        assert!(!caps.can_create_entity("ResearchLog"));
        assert!(!caps.can_delete_entity("ResearchLog"));
        
        // Test full capabilities
        let full_caps = ModuleCapabilities::full();
        assert!(full_caps.can_read_entity("ResearchLog"));
        assert!(full_caps.can_write_entity("ResearchLog"));
        assert!(full_caps.can_create_entity("ResearchLog"));
        assert!(full_caps.can_delete_entity("ResearchLog"));
    }
}

#[cfg(test)]
mod resource_limit_tests {
    use crate::resource_limits::ResourceLimits;
    use std::time::Duration;
    
    #[test]
    fn test_resource_limits_creation() {
        let limits = ResourceLimits {
            memory_bytes: 64 * 1024 * 1024, // 64MB
            storage_bytes: 10 * 1024 * 1024, // 10MB
            cpu_time_ms: 100,
            max_concurrent_ops: 10,
            entity_ops_per_minute: 1000,
            events_per_minute: 1000,
            table_elements: 10000,
            instances: 10,
        };
        
        assert_eq!(limits.memory_bytes, 64 * 1024 * 1024);
        assert_eq!(limits.cpu_time_limit(), Duration::from_millis(100));
    }
}

#[cfg(test)]
mod communication_tests {
    use crate::communication::{ModuleChannel, ModuleMessage, LogLevel};
    
    #[tokio::test]
    async fn test_module_channel() {
        let (host_side, mut module_side) = ModuleChannel::new(10);
        
        // Send from host to module
        host_side.to_module.send(ModuleMessage::Shutdown).await.unwrap();
        
        // Receive on module side
        if let Some(msg) = module_side.from_module.recv().await {
            assert!(matches!(msg, ModuleMessage::Shutdown));
        } else {
            panic!("Expected message");
        }
    }
    
    #[tokio::test]
    async fn test_log_message() {
        let (mut host_side, module_side) = ModuleChannel::new(10);
        
        // Send log from module to host
        let log_msg = ModuleMessage::Log {
            level: LogLevel::Info,
            message: "Test log message".to_string(),
        };
        
        module_side.to_module.send(log_msg).await.unwrap();
        
        // Receive on host side
        if let Some(ModuleMessage::Log { level, message }) = host_side.from_module.recv().await {
            assert!(matches!(level, LogLevel::Info));
            assert_eq!(message, "Test log message");
        } else {
            panic!("Expected log message");
        }
    }
}