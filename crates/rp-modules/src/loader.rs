//! Module loading for both native and WASM modules

use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;
use uuid::Uuid;
use chrono::Utc;

use wasmtime::{Engine, Module as WasmModule, Store, Linker};
use wasmtime_wasi::{WasiCtxBuilder, preview1::WasiP1Ctx};
use libloading::{Library, Symbol};

use crate::{
    ModuleMetadata, ModuleManifest, module::{ModuleType, ModuleStatus, ResearchModule},
    ModuleContext, ResourceLimits, resource_limits::WasmtimeResourceLimiter,
    Result, ModuleError
};

/// Module loader handles loading both native and WASM modules
pub struct ModuleLoader {
    /// WASM engine (shared across all WASM modules)
    wasm_engine: Engine,
    
    /// Loaded module instances
    instances: Arc<RwLock<Vec<Box<dyn ModuleInstance>>>>,
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
        let instance: Box<dyn ModuleInstance> = match manifest.module.module_type.as_str() {
            "native" => {
                self.load_native_module(path, manifest, context).await?
            }
            "wasm" => {
                self.load_wasm_module(path, manifest, context).await?
            }
            _ => {
                return Err(ModuleError::InvalidManifest(
                    format!("Unknown module type: {}", manifest.module.module_type)
                ));
            }
        };
        
        let id = instance.id();
        self.instances.write().push(instance);
        
        Ok(id)
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
    
    /// Load a WASM module
    async fn load_wasm_module(
        &self,
        path: &Path,
        manifest: ModuleManifest,
        context: ModuleContext,
    ) -> Result<Box<dyn ModuleInstance>> {
        let module_path = path.join(format!("{}.wasm", manifest.module.name));
        let wasm_bytes = tokio::fs::read(&module_path).await?;
        
        let module = WasmModule::from_binary(&self.wasm_engine, &wasm_bytes)
            .map_err(|e| ModuleError::WasmError(e.to_string()))?;
        
        let limits = manifest.to_resource_limits()?;
        let metadata = ModuleMetadata {
            id: context.instance_id,
            name: manifest.module.name.clone(),
            version: manifest.module.version.clone(),
            description: manifest.module.description.clone(),
            author: manifest.module.author.clone(),
            license: manifest.module.license.clone(),
            module_type: ModuleType::Wasm,
            loaded_at: Utc::now(),
            status: ModuleStatus::Loaded,
        };
        
        Ok(Box::new(WasmModuleInstance {
            id: context.instance_id,
            metadata,
            manifest,
            module,
            engine: self.wasm_engine.clone(),
            context,
            limits,
        }))
    }
    
    /// Get a loaded module instance
    pub fn get_instance(&self, id: &Uuid) -> Option<Box<dyn ModuleInstance>> {
        self.instances.read()
            .iter()
            .find(|inst| inst.id() == *id)
            .map(|inst| inst.clone_box())
    }
    
    /// Unload a module
    pub async fn unload_module(&self, id: Uuid) -> Result<()> {
        let mut instances = self.instances.write();
        
        if let Some(pos) = instances.iter().position(|inst| inst.id() == id) {
            let mut instance = instances.remove(pos);
            instance.shutdown().await?;
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
    
    /// Initialize the module
    async fn initialize(&mut self) -> Result<()>;
    
    /// Run the module
    async fn run(&mut self) -> Result<()>;
    
    /// Shutdown the module
    async fn shutdown(&mut self) -> Result<()>;
    
    /// Clone as boxed trait object
    fn clone_box(&self) -> Box<dyn ModuleInstance>;
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
        
        // Call the module's init function if it exists
        if let Ok(init_fn) = instance.get_typed_func::<(), ()>(&mut store, "_module_init") {
            init_fn.call_async(&mut store, ()).await
                .map_err(|e| ModuleError::InitializationError(format!("Module init failed: {}", e)))?;
        }
        
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
}

impl WasmModuleInstance {
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
        // We reconstruct it from the pointer
        let mut module = unsafe {
            let boxed_ptr = module_ptr as *mut Box<dyn ResearchModule>;
            *Box::from_raw(boxed_ptr)
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
}