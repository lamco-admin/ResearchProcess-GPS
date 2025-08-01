//! Simple test to verify WASM module can be loaded with Wasmtime

use wasmtime::*;
use wasmtime_wasi::preview1::{WasiP1Ctx, add_to_linker_sync};
use wasmtime_wasi::WasiCtxBuilder;

#[test]
fn test_wasm_module_loads() {
    // Path to the WASM module
    let wasm_path = "../../target/wasm32-wasip1/release/research_log_wasm.wasm";
    
    // Skip test if WASM file doesn't exist
    if !std::path::Path::new(wasm_path).exists() {
        eprintln!("WASM module not found at {}, skipping test", wasm_path);
        return;
    }
    
    // Create engine
    let engine = Engine::default();
    
    // Load WASM module
    let wasm_bytes = std::fs::read(wasm_path).expect("Failed to read WASM file");
    let module = Module::new(&engine, &wasm_bytes).expect("Failed to compile WASM module");
    
    // Create WASI context
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .build_p1();
    
    // Create store
    let mut store = Store::new(&engine, wasi_ctx);
    
    // Create linker and add WASI
    let mut linker = Linker::new(&engine);
    add_to_linker_sync(&mut linker, |ctx: &mut WasiP1Ctx| ctx).expect("Failed to add WASI to linker");
    
    // Add host functions that the WASM module expects
    linker.func_wrap("env", "host_emit_event", |_: Caller<'_, WasiP1Ctx>, _event_type: i32, _event_data: i32| {
        println!("host_emit_event called");
    }).expect("Failed to add host_emit_event");
    
    linker.func_wrap("env", "host_log", |_: Caller<'_, WasiP1Ctx>, _level: i32, _message: i32| {
        println!("host_log called");
    }).expect("Failed to add host_log");
    
    linker.func_wrap("env", "host_get_timestamp", |_: Caller<'_, WasiP1Ctx>| -> u64 {
        println!("host_get_timestamp called");
        1234567890
    }).expect("Failed to add host_get_timestamp");
    
    linker.func_wrap("env", "host_generate_uuid", |_: Caller<'_, WasiP1Ctx>| -> i32 {
        println!("host_generate_uuid called");
        0 // Would return pointer to string in real implementation
    }).expect("Failed to add host_generate_uuid");
    
    // Instantiate module
    let instance = linker.instantiate(&mut store, &module).expect("Failed to instantiate WASM module");
    
    // Try to find exported functions
    let exports = module.exports();
    println!("WASM module exports:");
    for export in exports {
        println!("  - {} ({:?})", export.name(), export.ty());
    }
    
    // Try to call initialize if it exists
    if let Ok(_initialize) = instance.get_typed_func::<(i32,), i32>(&mut store, "initialize") {
        println!("Found initialize function");
        // We'd need to pass proper string pointer here in real implementation
    } else {
        println!("Initialize function not found or has different signature");
    }
    
    println!("WASM module loaded successfully!");
}

#[test]
fn test_wasm_module_basic_structure() {
    let wasm_path = "../../target/wasm32-wasip1/release/research_log_wasm.wasm";
    
    if !std::path::Path::new(wasm_path).exists() {
        eprintln!("WASM module not found, skipping test");
        return;
    }
    
    let engine = Engine::default();
    let wasm_bytes = std::fs::read(wasm_path).expect("Failed to read WASM file");
    let module = Module::new(&engine, &wasm_bytes).expect("Failed to compile WASM module");
    
    // Check imports
    println!("\nWASM module imports:");
    for import in module.imports() {
        println!("  - {}::{} ({:?})", import.module(), import.name(), import.ty());
    }
    
    // Check memory requirements
    let mut has_memory = false;
    for export in module.exports() {
        if let ExternType::Memory(_) = export.ty() {
            has_memory = true;
            println!("\nModule exports memory");
        }
    }
    
    if !has_memory {
        println!("\nModule does not export memory");
    }
}