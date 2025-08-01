//! Macros for WASM module development

/// Macro to export a WASM module with C-style exports
#[macro_export]
macro_rules! export_wasm_module {
    ($module_type:ty) => {
        #[no_mangle]
        pub extern "C" fn create_module() {
            unsafe {
                $crate::wasm::set_module_instance(<$module_type>::new());
            }
        }
        
        // Ensure the module is created when the WASM module loads
        #[no_mangle]
        pub extern "C" fn _start() {
            create_module();
        }
    };
}

/// Macro to define a WASM module (simplified compared to native)
#[macro_export]
macro_rules! define_wasm_module {
    (
        $module_name:ident {
            info: $info_expr:expr,
            state: $state_type:ty,
            commands: {
                $($command:literal => |$args:ident, $state:ident| $handler:expr),* $(,)?
            },
            events: {
                $($event:literal => |$data:ident, $state:ident| $event_handler:expr),* $(,)?
            }
        }
    ) => {
        pub struct $module_name {
            info: $crate::ModuleInfo,
            context: Option<$crate::ModuleContext>,
            state: $state_type,
            module_state: $crate::ModuleState,
        }
        
        impl $module_name {
            pub fn new() -> Self {
                Self {
                    info: $info_expr,
                    context: None,
                    state: Default::default(),
                    module_state: $crate::ModuleState {
                        lifecycle: $crate::LifecycleEvent::Initializing,
                        commands_processed: 0,
                        events_received: 0,
                        resource_usage: $crate::ResourceUsage::default(),
                    },
                }
            }
            
            fn handle_command(&mut self, command: &str, args: serde_json::Value) -> $crate::Result<serde_json::Value> {
                match command {
                    $(
                        $command => {
                            let $args = args;
                            let $state = &mut self.state;
                            $handler
                        }
                    )*
                    _ => Err($crate::ModuleError::CommandError(
                        format!("Unknown command: {}", command)
                    )),
                }
            }
            
            fn handle_event_internal(&mut self, event_type: &str, data: serde_json::Value) -> $crate::Result<()> {
                match event_type {
                    $(
                        $event => {
                            let $data = data;
                            let $state = &mut self.state;
                            $event_handler
                        }
                    )*
                    _ => Ok(()), // Ignore unknown events
                }
            }
        }
        
        impl $crate::Module for $module_name {
            fn info(&self) -> $crate::ModuleInfo {
                self.info.clone()
            }
            
            fn initialize(&mut self, context: $crate::ModuleContext) -> $crate::Result<()> {
                self.context = Some(context);
                self.module_state.lifecycle = $crate::LifecycleEvent::Ready;
                $crate::wasm::log_message("info", &format!("{} module initialized", self.info.name));
                Ok(())
            }
            
            fn execute_command(&mut self, command: &str, args: serde_json::Value) -> $crate::Result<serde_json::Value> {
                self.module_state.lifecycle = $crate::LifecycleEvent::ProcessingCommand(command.to_string());
                let result = self.handle_command(command, args);
                self.module_state.commands_processed += 1;
                self.module_state.lifecycle = $crate::LifecycleEvent::Ready;
                result
            }
            
            fn handle_event(&mut self, event_type: &str, event_data: serde_json::Value) -> $crate::Result<()> {
                self.module_state.events_received += 1;
                self.handle_event_internal(event_type, event_data)
            }
            
            fn get_state(&self) -> $crate::ModuleState {
                self.module_state.clone()
            }
            
            fn shutdown(&mut self) -> $crate::Result<()> {
                self.module_state.lifecycle = $crate::LifecycleEvent::Shutdown;
                $crate::wasm::log_message("info", &format!("{} module shutdown", self.info.name));
                Ok(())
            }
        }
    };
}

/// Helper macro to emit an event in WASM modules
#[macro_export]
macro_rules! emit_event {
    ($event_type:expr, $data:expr) => {
        $crate::wasm::emit_event($event_type, &$data)
    };
}

/// Helper macro to log messages in WASM modules
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        $crate::wasm::log_message($level, &format!($($arg)*))
    };
}