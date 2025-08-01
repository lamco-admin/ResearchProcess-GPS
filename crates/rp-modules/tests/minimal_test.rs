//! Minimal test to isolate module system issues

#[test]
fn test_basic_compilation() {
    // This test does nothing but should compile
    assert_eq!(1 + 1, 2);
}

// Test just the loader creation without any WASM
#[test]
fn test_loader_creation_only() {
    use crate::loader::ModuleLoader;
    
    // Try to create a loader
    let result = ModuleLoader::new();
    
    // Check if it succeeded
    assert!(result.is_ok(), "Failed to create module loader: {:?}", result.err());
}