//! Utility functions for WASM

use wasm_bindgen::prelude::*;

/// Log to the browser console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn warn(s: &str);
}

/// Convert Rust error to JsValue
pub fn to_js_error(err: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&format!("Error: {}", err))
}
