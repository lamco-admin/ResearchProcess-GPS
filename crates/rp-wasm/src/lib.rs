//! WASM Bridge for ResearchProcess-GPS
//!
//! This crate provides the WebAssembly interface for running ResearchProcess-GPS
//! in the browser. It exposes the meta-model, schema system, and storage layer
//! to JavaScript.

use wasm_bindgen::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

mod entity;
mod relationship;
mod schema;
mod workspace;
mod storage;
mod utils;

pub use entity::*;
pub use relationship::*;
pub use schema::*;
pub use workspace::*;
pub use storage::*;

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages
    console_error_panic_hook::set_once();

    // Initialize tracing
    tracing_wasm::set_as_global_default();

    utils::log("ResearchProcess-GPS WASM initialized");
}

/// Get version information
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Global workspace instance
static WORKSPACE: Mutex<Option<Arc<workspace::WasmWorkspace>>> = Mutex::new(None);

/// Initialize a new workspace
#[wasm_bindgen]
pub async fn init_workspace(name: String) -> Result<JsValue, JsValue> {
    let ws = workspace::WasmWorkspace::new(name).await
        .map_err(|e| JsValue::from_str(&format!("Failed to initialize workspace: {}", e)))?;

    let ws_arc = Arc::new(ws);
    *WORKSPACE.lock().unwrap() = Some(ws_arc.clone());

    Ok(JsValue::from_str("Workspace initialized"))
}

/// Get the current workspace
#[wasm_bindgen]
pub fn get_workspace() -> Result<WasmWorkspace, JsValue> {
    let guard = WORKSPACE.lock().unwrap();
    match &*guard {
        Some(ws) => Ok((**ws).clone()),
        None => Err(JsValue::from_str("No workspace initialized. Call init_workspace() first.")),
    }
}
