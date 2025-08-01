//! Module loading for both native and WASM modules

use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;
use uuid::Uuid;
use chrono::Utc;

use wasmtime::{Engine, Module as WasmModule};

use crate::{
    ModuleMetadata, ModuleManifest, module::{ModuleType, ModuleStatus},
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
        _path: &Path,
        _manifest: ModuleManifest,
        _context: ModuleContext,
    ) -> Result<Box<dyn ModuleInstance>> {
        // Native module loading would use libloading
        // For now, return error as implementation is complex
        Err(ModuleError::LoadError(
            "Native module loading not yet implemented".to_string()
        ))
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
    manifest: ModuleManifest,
    module: WasmModule,
    engine: Engine,
    context: ModuleContext,
    limits: ResourceLimits,
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
        // Initialize WASM instance
        // This is simplified - real implementation would set up WASI, link functions, etc.
        Ok(())
    }
    
    async fn run(&mut self) -> Result<()> {
        // Run the WASM module
        // This would create store, instantiate module, call exports, etc.
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        // Clean shutdown
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn ModuleInstance> {
        // WASM instances can't be cloned directly
        // This would need to create a new instance
        unimplemented!("WASM module cloning not implemented")
    }
}