//! Native module support (Rust modules with full async support)

use crate::prelude::*;
use tokio::sync::mpsc;

pub mod macros;

/// Base implementation for native modules
pub struct NativeModuleBase {
    info: ModuleInfo,
    context: Option<ModuleContext>,
    state: ModuleState,
    command_router: CommandRouter,
    event_router: EventRouter,
    host_sender: Option<mpsc::Sender<ModuleMessage>>,
}

impl NativeModuleBase {
    /// Create a new native module base
    pub fn new(info: ModuleInfo) -> Self {
        Self {
            info,
            context: None,
            state: ModuleState {
                lifecycle: LifecycleEvent::Initializing,
                commands_processed: 0,
                events_received: 0,
                resource_usage: ResourceUsage::default(),
            },
            command_router: CommandRouter::new(),
            event_router: EventRouter::new(),
            host_sender: None,
        }
    }
    
    /// Get module info
    pub fn info(&self) -> &ModuleInfo {
        &self.info
    }
    
    /// Get module context
    pub fn context(&self) -> Option<&ModuleContext> {
        self.context.as_ref()
    }
    
    /// Get mutable command router
    pub fn command_router_mut(&mut self) -> &mut CommandRouter {
        &mut self.command_router
    }
    
    /// Get mutable event router
    pub fn event_router_mut(&mut self) -> &mut EventRouter {
        &mut self.event_router
    }
    
    /// Set host channel for communication
    pub fn set_host_channel(&mut self, sender: mpsc::Sender<ModuleMessage>) {
        self.host_sender = Some(sender);
    }
    
    /// Send a message to the host
    pub async fn send_to_host(&self, message: ModuleMessage) -> Result<()> {
        if let Some(sender) = &self.host_sender {
            sender.send(message).await
                .map_err(|e| ModuleError::CommunicationError(format!("Failed to send to host: {}", e)))?;
            Ok(())
        } else {
            Err(ModuleError::CommunicationError("No host channel configured".to_string()))
        }
    }
    
    /// Emit an event
    pub async fn emit_event(&self, event_type: impl Into<String>, data: Value) -> Result<()> {
        let event = Event {
            event_type: event_type.into(),
            data,
            timestamp: chrono::Utc::now().timestamp() as u64,
            source: Some(self.info.name.clone()),
        };
        
        self.send_to_host(ModuleMessage::Event { 
            event: serde_json::to_value(event)
                .map_err(|e| ModuleError::SerializationError(e))? 
        }).await
    }
    
    /// Log a message
    pub async fn log(&self, level: log::Level, message: impl Into<String>) -> Result<()> {
        self.send_to_host(ModuleMessage::LogMessage {
            level: level.to_string(),
            message: message.into(),
        }).await
    }
    
    /// Update module state
    pub fn update_state<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ModuleState),
    {
        f(&mut self.state);
    }
}

/// Helper trait for native module implementation
#[async_trait]
pub trait NativeModule: Module {
    /// Get the base module
    fn base(&self) -> &NativeModuleBase;
    
    /// Get the base module mutably
    fn base_mut(&mut self) -> &mut NativeModuleBase;
    
    /// Called when module is loaded
    async fn on_load(&mut self) -> Result<()> {
        Ok(())
    }
    
    /// Called when module is unloaded
    async fn on_unload(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Module runner for native modules
pub struct NativeModuleRunner<M: NativeModule> {
    module: M,
    module_channel: mpsc::Receiver<ModuleMessage>,
    host_channel: mpsc::Sender<ModuleMessage>,
}

impl<M: NativeModule> NativeModuleRunner<M> {
    /// Create a new module runner
    pub fn new(
        mut module: M,
        module_channel: mpsc::Receiver<ModuleMessage>,
        host_channel: mpsc::Sender<ModuleMessage>,
    ) -> Self {
        module.base_mut().set_host_channel(host_channel.clone());
        Self {
            module,
            module_channel,
            host_channel,
        }
    }
    
    /// Run the module
    pub async fn run(mut self) -> Result<()> {
        // Initialize
        self.module.on_load().await?;
        
        // Main message loop
        while let Some(message) = self.module_channel.recv().await {
            match message {
                ModuleMessage::Initialize { actor_id, config } => {
                    let context = ModuleContext {
                        instance_id: Uuid::new_v4(),
                        actor_id,
                        config,
                        resource_limits: ResourceLimits::default(),
                        capabilities: vec![],
                    };
                    
                    self.module.initialize(context).await?;
                    self.module.base_mut().update_state(|state| {
                        state.lifecycle = LifecycleEvent::Ready;
                    });
                }
                
                ModuleMessage::CommandRequest { id, command, args } => {
                    self.module.base_mut().update_state(|state| {
                        state.lifecycle = LifecycleEvent::ProcessingCommand(command.clone());
                    });
                    
                    let result = self.module.execute_command(&command, args).await;
                    
                    self.module.base_mut().update_state(|state| {
                        state.commands_processed += 1;
                        state.lifecycle = LifecycleEvent::CommandComplete(command.clone());
                    });
                    
                    let response = match result {
                        Ok(value) => ModuleMessage::CommandResponse { 
                            id, 
                            result: Ok(value) 
                        },
                        Err(e) => ModuleMessage::CommandResponse { 
                            id, 
                            result: Err(e.to_string()) 
                        },
                    };
                    
                    self.host_channel.send(response).await
                        .map_err(|e| ModuleError::CommunicationError(format!("Failed to send response: {}", e)))?;
                }
                
                ModuleMessage::Event { event } => {
                    if let Ok(event_obj) = serde_json::from_value::<Event>(event.clone()) {
                        self.module.base_mut().update_state(|state| {
                            state.events_received += 1;
                        });
                        
                        let _ = self.module.handle_event(&event_obj.event_type, event_obj.data).await;
                    }
                }
                
                ModuleMessage::Shutdown => {
                    self.module.base_mut().update_state(|state| {
                        state.lifecycle = LifecycleEvent::ShuttingDown;
                    });
                    
                    self.module.shutdown().await?;
                    self.module.on_unload().await?;
                    
                    self.module.base_mut().update_state(|state| {
                        state.lifecycle = LifecycleEvent::Shutdown;
                    });
                    
                    break;
                }
                
                _ => {
                    // Ignore other message types
                }
            }
        }
        
        Ok(())
    }
}

/// Export the module runner entrypoint
#[macro_export]
macro_rules! export_native_module {
    ($module_type:ty) => {
        #[no_mangle]
        pub extern "C" fn create_module() -> Box<dyn $crate::Module> {
            Box::new(<$module_type>::new())
        }
        
        #[no_mangle]
        pub extern "C" fn module_sdk_version() -> &'static str {
            $crate::SDK_VERSION
        }
    };
}