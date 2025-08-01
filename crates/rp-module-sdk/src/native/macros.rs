//! Macros for native module development

/// Macro to implement a simple command handler
#[macro_export]
macro_rules! command_handler {
    ($name:ident, |$args:ident| $body:expr) => {
        struct $name;
        
        #[async_trait::async_trait]
        impl $crate::CommandHandler for $name {
            async fn handle(&mut self, $args: serde_json::Value) -> $crate::Result<serde_json::Value> {
                $body
            }
        }
    };
}

/// Macro to implement a simple event handler
#[macro_export]
macro_rules! event_handler {
    ($name:ident, |$data:ident| $body:expr) => {
        struct $name;
        
        #[async_trait::async_trait]
        impl $crate::EventHandler for $name {
            async fn handle(&mut self, $data: serde_json::Value) -> $crate::Result<()> {
                $body
            }
        }
    };
}

/// Macro to define a native module
#[macro_export]
macro_rules! define_native_module {
    (
        $module_name:ident {
            info: $info_expr:expr,
            state: $state_type:ty,
            commands: {
                $($command:literal => $handler:expr),* $(,)?
            },
            events: {
                $($event:literal => $event_handler:expr),* $(,)?
            }
        }
    ) => {
        pub struct $module_name {
            base: $crate::native::NativeModuleBase,
            state: $state_type,
        }
        
        impl $module_name {
            pub fn new() -> Self {
                let mut base = $crate::native::NativeModuleBase::new($info_expr);
                
                // Register command handlers
                $(
                    base.command_router_mut().register($command, $handler);
                )*
                
                // Register event handlers
                $(
                    base.event_router_mut().register($event, $event_handler);
                )*
                
                Self {
                    base,
                    state: Default::default(),
                }
            }
        }
        
        #[async_trait::async_trait]
        impl $crate::Module for $module_name {
            fn info(&self) -> $crate::ModuleInfo {
                self.base.info().clone()
            }
            
            async fn initialize(&mut self, context: $crate::ModuleContext) -> $crate::Result<()> {
                self.base.context = Some(context);
                self.base.update_state(|state| {
                    state.lifecycle = $crate::LifecycleEvent::Ready;
                });
                Ok(())
            }
            
            async fn execute_command(&mut self, command: &str, args: serde_json::Value) -> $crate::Result<serde_json::Value> {
                self.base.command_router_mut().execute(command, args).await
            }
            
            async fn handle_event(&mut self, event_type: &str, event_data: serde_json::Value) -> $crate::Result<()> {
                self.base.event_router_mut().handle(event_type, event_data).await
            }
            
            fn get_state(&self) -> $crate::ModuleState {
                self.base.state.clone()
            }
            
            async fn shutdown(&mut self) -> $crate::Result<()> {
                self.base.update_state(|state| {
                    state.lifecycle = $crate::LifecycleEvent::Shutdown;
                });
                Ok(())
            }
        }
        
        #[async_trait::async_trait]
        impl $crate::native::NativeModule for $module_name {
            fn base(&self) -> &$crate::native::NativeModuleBase {
                &self.base
            }
            
            fn base_mut(&mut self) -> &mut $crate::native::NativeModuleBase {
                &mut self.base
            }
        }
    };
}