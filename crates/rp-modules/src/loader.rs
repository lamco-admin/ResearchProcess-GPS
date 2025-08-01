//! Module loading for both native and WASM modules

use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::Utc;

use wasmtime::{Engine, Module as WasmModule, Store, Linker, Instance};
use wasmtime_wasi::{WasiCtxBuilder, preview1::WasiP1Ctx};
use libloading::{Library, Symbol};

use crate::{
    ModuleMetadata, ModuleManifest, module::{ModuleType, ModuleStatus, ResearchModule},
    ModuleContext, ResourceLimits, resource_limits::WasmtimeResourceLimiter,
    Result, ModuleError, ModuleMessage
};

/// Module loader handles loading both native and WASM modules
#[derive(Clone)]
pub struct ModuleLoader {
    /// WASM engine (shared across all WASM modules)
    wasm_engine: Engine,
    
    /// Loaded module instances
    instances: Arc<RwLock<Vec<Arc<Mutex<Box<dyn ModuleInstance>>>>>>,
}

/// Information about a loaded module
pub struct ModuleInfo {
    pub metadata: ModuleMetadata,
}

impl ModuleLoader {
    /// Create a new module loader
    pub fn new() -> Result<Self> {
        let mut config = wasmtime::Config::new();
        config.wasm_threads(true);
        config.async_support(true);
        
        let wasm_engine = Engine::new(&config)?;
        
        Ok(Self {
            wasm_engine,
            instances: Arc::new(RwLock::new(Vec::new())),
        })
    }
    
    /// Load a WASM module directly from file path
    pub async fn load_wasm_module(
        &self,
        wasm_path: impl AsRef<Path>,
        context: ModuleContext,
        limits: ResourceLimits,
    ) -> Result<Uuid> {
        let wasm_path = wasm_path.as_ref();
        let wasm_bytes = tokio::fs::read(&wasm_path).await?;
        
        let module = WasmModule::from_binary(&self.wasm_engine, &wasm_bytes)
            .map_err(|e| ModuleError::WasmError(e.to_string()))?;
        
        // Create metadata
        let metadata = ModuleMetadata {
            id: context.instance_id,
            name: wasm_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string(),
            version: "0.1.0".to_string(),
            description: "WASM module".to_string(),
            author: "Unknown".to_string(),
            license: "Unknown".to_string(),
            module_type: ModuleType::Wasm,
            loaded_at: Utc::now(),
            status: ModuleStatus::Loaded,
        };
        
        let manifest = ModuleManifest::default();
        
        let instance = Box::new(WasmModuleInstance {
            id: context.instance_id,
            metadata,
            manifest,
            module,
            engine: self.wasm_engine.clone(),
            context,
            limits,
            runtime: Arc::new(Mutex::new(None)),
        });
        
        let id = instance.id();
        self.instances.write().push(Arc::new(Mutex::new(instance)));
        
        Ok(id)
    }
    
    /// Check if a module is loaded
    pub fn is_loaded(&self, id: &Uuid) -> bool {
        self.instances.read().iter().any(|inst| {
            inst.try_lock().map(|i| i.id() == *id).unwrap_or(false)
        })
    }
    
    /// Get module information
    pub fn get_module(&self, id: &Uuid) -> Option<ModuleInfo> {
        self.instances.read()
            .iter()
            .find(|inst| {
                inst.try_lock().map(|i| i.id() == *id).unwrap_or(false)
            })
            .and_then(|inst| {
                inst.try_lock().ok().map(|i| ModuleInfo {
                    metadata: i.metadata().clone(),
                })
            })
    }
    
    /// Execute a command on a module
    pub async fn execute_command(
        &self,
        id: &Uuid,
        command: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value> {
        // For WASM modules, we need to call the execute_command export directly
        // Check if this is a WASM module by trying to downcast
        let instances = self.instances.read();
        let instance = instances.iter()
            .find(|inst| {
                inst.try_lock().map(|i| i.id() == *id).unwrap_or(false)
            })
            .ok_or_else(|| ModuleError::NotFound(id.to_string()))?;
        
        // Clone the Arc to avoid holding the lock
        let instance_arc = instance.clone();
        drop(instances);
        
        // Try to execute command directly on WASM module
        let mut instance_guard = instance_arc.lock().await;
        if let Some(wasm_instance) = instance_guard.as_any_mut().downcast_mut::<WasmModuleInstance>() {
            return wasm_instance.execute_wasm_command(command, args).await;
        }
        drop(instance_guard);
        
        // For native modules, use the channel approach
        // Create a unique request ID
        let request_id = Uuid::new_v4();
        
        // Send command request to module
        let message = ModuleMessage::CommandRequest { 
            id: request_id,
            command: command.to_string(), 
            args 
        };
        
        // Get the instance again to send the message
        let instances = self.instances.read();
        let instance = instances.iter()
            .find(|inst| {
                inst.try_lock().map(|i| i.id() == *id).unwrap_or(false)
            })
            .ok_or_else(|| ModuleError::NotFound(id.to_string()))?;
        
        let instance_guard = instance.try_lock()
            .map_err(|_| ModuleError::CommunicationError("Module is locked".to_string()))?;
        
        // Send message to module through its channel
        instance_guard.context().host_channel.send(message).await
            .map_err(|e| ModuleError::CommunicationError(format!("Failed to send command: {}", e)))?;
        drop(instance_guard);
        drop(instances);
        
        // For now, return a placeholder response since we don't have a response channel yet
        // TODO: Implement proper command response handling
        Ok(serde_json::json!({
            "status": "sent",
            "request_id": request_id
        }))
    }
    
    /// Load a module from path
    pub async fn load_module(
        &self,
        path: impl AsRef<Path>,
        context: ModuleContext,
    ) -> Result<Uuid> {
        let path = path.as_ref();
        
        // Load manifest
        let manifest_path = path.join("module.toml");
        let manifest = ModuleManifest::from_file(&manifest_path).await?;
        
        // Create instance based on module type
        match manifest.module.module_type.as_str() {
            "native" => {
                let instance = self.load_native_module(path, manifest, context).await?;
                let id = instance.id();
                self.instances.write().push(Arc::new(Mutex::new(instance)));
                Ok(id)
            }
            "wasm" => {
                let wasm_path = path.join(format!("{}.wasm", manifest.module.name));
                let limits = manifest.to_resource_limits()?;
                self.load_wasm_module(wasm_path, context, limits).await
            }
            _ => {
                Err(ModuleError::InvalidManifest(
                    format!("Unknown module type: {}", manifest.module.module_type)
                ))
            }
        }
    }
    
    /// Load a native Rust module
    async fn load_native_module(
        &self,
        path: &Path,
        manifest: ModuleManifest,
        context: ModuleContext,
    ) -> Result<Box<dyn ModuleInstance>> {
        // Build the module library path
        // Try multiple naming conventions
        let lib_names = vec![
            format!("lib{}.so", manifest.module.name),
            format!("lib{}.so", manifest.module.name.replace("-", "_")),
            format!("lib{}_module.so", manifest.module.name.replace("-", "_")),
        ];
        
        let lib_path = lib_names.iter()
            .map(|name| path.join(name))
            .find(|p| p.exists())
            .ok_or_else(|| ModuleError::LoadError(
                format!("Native module library not found. Tried: {:?}", lib_names)
            ))?;
        
        // Load the dynamic library
        let library = unsafe {
            Library::new(&lib_path)
                .map_err(|e| ModuleError::LoadError(
                    format!("Failed to load native module: {}", e)
                ))?
        };
        
        // Create metadata
        let metadata = ModuleMetadata {
            id: context.instance_id,
            name: manifest.module.name.clone(),
            version: manifest.module.version.clone(),
            description: manifest.module.description.clone(),
            author: manifest.module.author.clone(),
            license: manifest.module.license.clone(),
            module_type: ModuleType::Native,
            loaded_at: Utc::now(),
            status: ModuleStatus::Loaded,
        };
        
        Ok(Box::new(NativeModuleInstance {
            id: context.instance_id,
            metadata,
            manifest,
            library: Arc::new(library),
            context,
            module: None,
        }))
    }
    
    
    /// Get a loaded module instance
    pub fn get_instance(&self, _id: &Uuid) -> Option<Box<dyn ModuleInstance>> {
        // This method might need to be removed or rethought since we can't clone trait objects
        None
    }
    
    /// Initialize a module
    pub async fn initialize_module(&self, id: &Uuid) -> Result<()> {
        let instances = self.instances.read();
        let instance = instances.iter()
            .find(|inst| {
                inst.try_lock().map(|i| i.id() == *id).unwrap_or(false)
            })
            .ok_or_else(|| ModuleError::NotFound(id.to_string()))?;
        
        // Clone the Arc to avoid holding the lock
        let instance_arc = instance.clone();
        drop(instances);
        
        // Get mutable access to the specific instance
        let mut instance_guard = instance_arc.lock().await;
        instance_guard.initialize().await?;
        
        // Update status to Running
        match instance_guard.metadata_mut() {
            Some(metadata) => metadata.status = ModuleStatus::Running,
            None => return Err(ModuleError::InitializationError("Failed to update module status".to_string())),
        }
        
        Ok(())
    }
    
    /// Run a module
    pub async fn run_module(&self, id: &Uuid) -> Result<()> {
        // Find and clone the instance Arc
        let instance_arc = {
            let instances = self.instances.read();
            instances.iter()
                .find(|inst| {
                    inst.try_lock().map(|i| i.id() == *id).unwrap_or(false)
                })
                .ok_or_else(|| ModuleError::NotFound(id.to_string()))?
                .clone()
        };
        
        // Now run without holding any outer locks
        let mut instance_guard = instance_arc.lock().await;
        instance_guard.run().await
    }
    
    /// Unload a module
    pub async fn unload_module(&self, id: Uuid) -> Result<()> {
        let mut instances = self.instances.write();
        
        if let Some(pos) = instances.iter().position(|inst| {
            inst.try_lock().map(|i| i.id() == id).unwrap_or(false)
        }) {
            let instance = instances.remove(pos);
            let mut instance_guard = instance.lock().await;
            instance_guard.shutdown().await?;
        }
        
        Ok(())
    }
}

/// Trait for module instances
#[async_trait::async_trait]
pub trait ModuleInstance: Send + Sync {
    /// Get instance ID
    fn id(&self) -> Uuid;
    
    /// Get metadata
    fn metadata(&self) -> &ModuleMetadata;
    
    /// Get mutable metadata
    fn metadata_mut(&mut self) -> Option<&mut ModuleMetadata>;
    
    /// Initialize the module
    async fn initialize(&mut self) -> Result<()>;
    
    /// Run the module
    async fn run(&mut self) -> Result<()>;
    
    /// Shutdown the module
    async fn shutdown(&mut self) -> Result<()>;
    
    /// Clone as boxed trait object
    fn clone_box(&self) -> Box<dyn ModuleInstance>;
    
    /// Get the module context
    fn context(&self) -> &ModuleContext;
    
    /// Get as Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
    
    /// Get as mutable Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// WASM module runtime state
struct WasmRuntime {
    store: Store<StoreData>,
    instance: Instance,
}

/// WASM module instance
struct WasmModuleInstance {
    id: Uuid,
    metadata: ModuleMetadata,
    #[allow(dead_code)]
    manifest: ModuleManifest,
    module: WasmModule,
    engine: Engine,
    context: ModuleContext,
    limits: ResourceLimits,
    runtime: Arc<Mutex<Option<WasmRuntime>>>,
}

/// Native module instance
struct NativeModuleInstance {
    id: Uuid,
    metadata: ModuleMetadata,
    #[allow(dead_code)]
    manifest: ModuleManifest,
    library: Arc<Library>,
    context: ModuleContext,
    module: Option<Box<dyn ResearchModule>>,
}

#[async_trait::async_trait]
impl ModuleInstance for WasmModuleInstance {
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn metadata(&self) -> &ModuleMetadata {
        &self.metadata
    }
    
    fn metadata_mut(&mut self) -> Option<&mut ModuleMetadata> {
        Some(&mut self.metadata)
    }
    
    async fn initialize(&mut self) -> Result<()> {
        // Create a WASI context with limited capabilities
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .build_p1();
        
        // Create store with resource limiter
        let mut store = Store::new(&self.engine, StoreData {
            wasi,
            limits: WasmtimeResourceLimiter::new(self.limits.clone()),
            context: self.context.clone(),
        });
        
        // Apply resource limiter
        store.limiter(|data| &mut data.limits);
        
        // Create linker and add WASI preview1
        let mut linker = Linker::new(&self.engine);
        wasmtime_wasi::preview1::add_to_linker_async(&mut linker, |data: &mut StoreData| &mut data.wasi)
            .map_err(|e| ModuleError::WasmError(format!("Failed to add WASI to linker: {}", e)))?;
        
        // Add host functions for module communication
        self.add_host_functions(&mut linker)?;
        
        // Instantiate the module
        let instance = linker.instantiate_async(&mut store, &self.module).await
            .map_err(|e| ModuleError::WasmError(format!("Failed to instantiate module: {}", e)))?;
        
        // Call the module's initialize function if it exists
        if let Ok(init_fn) = instance.get_typed_func::<(i32, i32), i32>(&mut store, "initialize") {
            // Pass actor_id as a string
            let actor_id = self.context.actor_id.to_string();
            let actor_id_bytes = actor_id.as_bytes();
            
            // Allocate memory for the string in WASM
            let memory = instance.get_memory(&mut store, "memory")
                .ok_or_else(|| ModuleError::WasmError("No memory export found".to_string()))?;
            
            // For now, use a fixed offset - in production, we'd call __wbindgen_malloc
            let ptr = 1024i32; // Safe offset for small strings
            memory.write(&mut store, ptr as usize, actor_id_bytes)
                .map_err(|e| ModuleError::WasmError(format!("Failed to write to WASM memory: {}", e)))?;
            
            let result = init_fn.call_async(&mut store, (ptr, actor_id_bytes.len() as i32)).await
                .map_err(|e| ModuleError::InitializationError(format!("Module init failed: {}", e)))?;
            
            if result == 0 {
                return Err(ModuleError::InitializationError("Module initialization returned error".to_string()));
            }
        }
        
        // Store the runtime
        *self.runtime.lock().await = Some(WasmRuntime {
            store,
            instance,
        });
        
        Ok(())
    }
    
    async fn run(&mut self) -> Result<()> {
        // WASM modules typically run through exported functions
        // The actual implementation would process messages and call appropriate exports
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        // Call module's cleanup function if it exists
        // In a real implementation, we'd maintain the store and instance
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn ModuleInstance> {
        // WASM instances can't be cloned directly
        unimplemented!("WASM module cloning not implemented")
    }
    
    fn context(&self) -> &ModuleContext {
        &self.context
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl WasmModuleInstance {
    /// Execute a command on the WASM module
    async fn execute_wasm_command(&mut self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        let mut runtime_guard = self.runtime.lock().await;
        let runtime = runtime_guard.as_mut()
            .ok_or_else(|| ModuleError::ExecutionError("Module not initialized".to_string()))?;
        
        let store = &mut runtime.store;
        let instance = &runtime.instance;
        
        // Get the execute_command export
        let execute_fn = instance.get_typed_func::<(i32, i32, i32, i32), i32>(&mut *store, "execute_command")
            .map_err(|e| ModuleError::ExecutionError(format!("execute_command export not found: {}", e)))?;
        
        // Allocate memory for command and args strings
        let memory = instance.get_memory(&mut *store, "memory")
            .ok_or_else(|| ModuleError::WasmError("No memory export found".to_string()))?;
        
        // Convert to strings
        let command_bytes = command.as_bytes();
        let args_str = args.to_string();
        let args_bytes = args_str.as_bytes();
        
        // Simple allocation - in production we'd call __wbindgen_malloc
        let cmd_ptr = 2048i32;
        let args_ptr = (2048 + command_bytes.len() + 16) as i32; // Leave some padding
        
        // Write to memory
        memory.write(&mut *store, cmd_ptr as usize, command_bytes)
            .map_err(|e| ModuleError::WasmError(format!("Failed to write command: {}", e)))?;
        memory.write(&mut *store, args_ptr as usize, args_bytes)
            .map_err(|e| ModuleError::WasmError(format!("Failed to write args: {}", e)))?;
        
        // Call the function
        let result_ptr = execute_fn.call_async(&mut *store, (
            cmd_ptr, 
            command_bytes.len() as i32,
            args_ptr,
            args_bytes.len() as i32
        )).await
            .map_err(|e| ModuleError::ExecutionError(format!("Command execution failed: {}", e)))?;
        
        // Read the result string
        if result_ptr == 0 {
            return Err(ModuleError::ExecutionError("Command returned null".to_string()));
        }
        
        // For C-style strings, we need to find the null terminator
        let mut result_bytes = Vec::new();
        let mut offset = 0;
        loop {
            let mut byte = [0u8; 1];
            memory.read(&mut *store, (result_ptr + offset) as usize, &mut byte)
                .map_err(|e| ModuleError::WasmError(format!("Failed to read result byte: {}", e)))?;
            
            if byte[0] == 0 {
                break;
            }
            
            result_bytes.push(byte[0]);
            offset += 1;
            
            // Safety limit to prevent infinite loops
            if offset > 1_000_000 {
                return Err(ModuleError::ExecutionError("Result string too long".to_string()));
            }
        }
        
        let result_str = String::from_utf8(result_bytes)
            .map_err(|e| ModuleError::ExecutionError(format!("Invalid UTF-8 in result: {}", e)))?;
        
        // Free the result string if the module exports a free_string function
        if let Ok(free_fn) = instance.get_typed_func::<i32, ()>(&mut *store, "free_string") {
            // Ignore errors from free_string - it's a best-effort cleanup
            let _ = free_fn.call_async(&mut *store, result_ptr).await;
        }
        
        // Parse JSON result
        serde_json::from_str(&result_str)
            .map_err(|e| ModuleError::ExecutionError(format!("Invalid JSON in result: {}", e)))
    }
    /// Add host functions for module communication
    fn add_host_functions(&self, linker: &mut Linker<StoreData>) -> Result<()> {
        // Add function for sending messages to host
        linker.func_wrap_async("env", "send_message", 
            |_caller: wasmtime::Caller<'_, StoreData>, (_ptr, _len): (u32, u32)| {
                Box::new(async move {
                    // In a real implementation, this would:
                    // 1. Read message data from WASM memory
                    // 2. Deserialize into ModuleMessage
                    // 3. Send via context.host_channel
                    Ok(())
                })
            }
        ).map_err(|e| ModuleError::WasmError(format!("Failed to add host function: {}", e)))?;
        
        // Add more host functions as needed for:
        // - Logging
        // - Entity operations
        // - Event handling
        // - etc.
        
        Ok(())
    }
}

/// Store data for WASM instances
struct StoreData {
    wasi: WasiP1Ctx,
    limits: WasmtimeResourceLimiter,
    #[allow(dead_code)]
    context: ModuleContext,
}

#[async_trait::async_trait]
impl ModuleInstance for NativeModuleInstance {
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn metadata(&self) -> &ModuleMetadata {
        &self.metadata
    }
    
    fn metadata_mut(&mut self) -> Option<&mut ModuleMetadata> {
        Some(&mut self.metadata)
    }
    
    async fn initialize(&mut self) -> Result<()> {
        // Native modules export a create function that returns a Box<dyn ResearchModule>
        // We use a raw pointer for FFI compatibility
        type ModuleCreateFn = unsafe extern "C" fn() -> *mut std::ffi::c_void;
        
        let create_fn: Symbol<ModuleCreateFn> = unsafe {
            self.library.get(b"_create_module\0")
                .map_err(|e| ModuleError::LoadError(
                    format!("Failed to find _create_module function: {}", e)
                ))?
        };
        
        // Create the module instance
        let module_ptr = unsafe { create_fn() };
        if module_ptr.is_null() {
            return Err(ModuleError::LoadError(
                "Module creation function returned null".to_string()
            ));
        }
        
        // The module returns a Box<dyn ResearchModule> as a raw pointer
        // We need to cast it back from c_void
        let mut module = unsafe {
            // Cast the c_void pointer to a raw pointer to Box<dyn ResearchModule>
            let boxed_trait = module_ptr as *mut Box<dyn ResearchModule>;
            // Dereference to get the Box<dyn ResearchModule>
            *Box::from_raw(boxed_trait)
        };
        
        // Initialize the module with context
        module.initialize(self.context.clone()).await
            .map_err(|e| ModuleError::InitializationError(e.to_string()))?;
        
        self.module = Some(module);
        Ok(())
    }
    
    async fn run(&mut self) -> Result<()> {
        // Native modules run continuously via message handling
        // This method would typically start a message processing loop
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        if let Some(mut module) = self.module.take() {
            module.shutdown().await
                .map_err(|e| ModuleError::ShutdownError(e.to_string()))?;
        }
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn ModuleInstance> {
        // Native modules can't be cloned directly
        unimplemented!("Native module cloning not implemented")
    }
    
    fn context(&self) -> &ModuleContext {
        &self.context
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}