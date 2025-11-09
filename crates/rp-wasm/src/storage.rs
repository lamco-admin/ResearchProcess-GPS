//! Storage abstraction for browser (IndexedDB)

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{IdbDatabase, IdbFactory, IdbOpenDbRequest, IdbObjectStore, IdbTransactionMode};

use crate::utils::to_js_error;

const DB_VERSION: u32 = 1;
const ENTITIES_STORE: &str = "entities";
const RELATIONSHIPS_STORE: &str = "relationships";
const SCHEMAS_STORE: &str = "schemas";
const WORKSPACE_STORE: &str = "workspace";

/// Storage manager for browser persistence using IndexedDB
#[wasm_bindgen]
pub struct WasmStorage {
    db_name: String,
}

#[wasm_bindgen]
impl WasmStorage {
    /// Create a new storage instance
    #[wasm_bindgen(constructor)]
    pub fn new(db_name: String) -> WasmStorage {
        WasmStorage { db_name }
    }

    /// Initialize the database
    #[wasm_bindgen]
    pub async fn init(&self) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
        let idb_factory: IdbFactory = window
            .indexed_db()
            .map_err(to_js_error)?
            .ok_or_else(|| JsValue::from_str("IndexedDB not supported"))?;

        let open_request: IdbOpenDbRequest = idb_factory
            .open_with_u32(&self.db_name, DB_VERSION)
            .map_err(to_js_error)?;

        // Set up upgrade handler
        let on_upgrade_needed = Closure::wrap(Box::new(move |event: &web_sys::Event| {
            crate::utils::log("Upgrading database schema...");

            let target = event.target().expect("Event should have target");
            let request = target
                .dyn_into::<IdbOpenDbRequest>()
                .expect("Event target should be IdbOpenDbRequest");
            let db = request.result().unwrap().dyn_into::<IdbDatabase>().unwrap();

            // Create object stores
            if !db.object_store_names().contains(ENTITIES_STORE) {
                let _ = db.create_object_store(ENTITIES_STORE);
            }
            if !db.object_store_names().contains(RELATIONSHIPS_STORE) {
                let _ = db.create_object_store(RELATIONSHIPS_STORE);
            }
            if !db.object_store_names().contains(SCHEMAS_STORE) {
                let _ = db.create_object_store(SCHEMAS_STORE);
            }
            if !db.object_store_names().contains(WORKSPACE_STORE) {
                let _ = db.create_object_store(WORKSPACE_STORE);
            }
        }) as Box<dyn FnMut(&web_sys::Event)>);

        open_request.set_onupgradeneeded(Some(on_upgrade_needed.as_ref().unchecked_ref()));
        on_upgrade_needed.forget();

        // Wait for database to open
        JsFuture::from(open_request).await?;

        crate::utils::log(&format!("IndexedDB '{}' initialized successfully", self.db_name));
        Ok(())
    }

    /// Save workspace to storage
    #[wasm_bindgen]
    pub async fn save_workspace(&self, workspace_json: JsValue) -> Result<(), JsValue> {
        let db = self.open_db().await?;

        let transaction = db
            .transaction_with_str_and_mode(WORKSPACE_STORE, IdbTransactionMode::Readwrite)
            .map_err(to_js_error)?;

        let store = transaction
            .object_store(WORKSPACE_STORE)
            .map_err(to_js_error)?;

        let request = store
            .put_with_key(&workspace_json, &JsValue::from_str("current"))
            .map_err(to_js_error)?;

        JsFuture::from(request).await?;

        crate::utils::log("Workspace saved to IndexedDB");
        Ok(())
    }

    /// Load workspace from storage
    #[wasm_bindgen]
    pub async fn load_workspace(&self) -> Result<JsValue, JsValue> {
        let db = self.open_db().await?;

        let transaction = db
            .transaction_with_str(WORKSPACE_STORE)
            .map_err(to_js_error)?;

        let store = transaction
            .object_store(WORKSPACE_STORE)
            .map_err(to_js_error)?;

        let request = store
            .get(&JsValue::from_str("current"))
            .map_err(to_js_error)?;

        let result = JsFuture::from(request).await?;

        if result.is_undefined() || result.is_null() {
            return Err(JsValue::from_str("No saved workspace found"));
        }

        crate::utils::log("Workspace loaded from IndexedDB");
        Ok(result)
    }

    /// Save an entity to storage
    #[wasm_bindgen]
    pub async fn save_entity(&self, id: String, entity_json: JsValue) -> Result<(), JsValue> {
        let db = self.open_db().await?;

        let transaction = db
            .transaction_with_str_and_mode(ENTITIES_STORE, IdbTransactionMode::Readwrite)
            .map_err(to_js_error)?;

        let store = transaction
            .object_store(ENTITIES_STORE)
            .map_err(to_js_error)?;

        let request = store
            .put_with_key(&entity_json, &JsValue::from_str(&id))
            .map_err(to_js_error)?;

        JsFuture::from(request).await?;
        Ok(())
    }

    /// Load an entity from storage
    #[wasm_bindgen]
    pub async fn load_entity(&self, id: String) -> Result<JsValue, JsValue> {
        let db = self.open_db().await?;

        let transaction = db
            .transaction_with_str(ENTITIES_STORE)
            .map_err(to_js_error)?;

        let store = transaction
            .object_store(ENTITIES_STORE)
            .map_err(to_js_error)?;

        let request = store
            .get(&JsValue::from_str(&id))
            .map_err(to_js_error)?;

        JsFuture::from(request).await
    }

    /// Delete workspace and all data
    #[wasm_bindgen]
    pub async fn clear(&self) -> Result<(), JsValue> {
        let db = self.open_db().await?;

        let stores = [ENTITIES_STORE, RELATIONSHIPS_STORE, SCHEMAS_STORE, WORKSPACE_STORE];

        for store_name in &stores {
            let transaction = db
                .transaction_with_str_and_mode(store_name, IdbTransactionMode::Readwrite)
                .map_err(to_js_error)?;

            let store = transaction
                .object_store(store_name)
                .map_err(to_js_error)?;

            let request = store.clear().map_err(to_js_error)?;
            JsFuture::from(request).await?;
        }

        crate::utils::log("Storage cleared");
        Ok(())
    }
}

impl WasmStorage {
    /// Open the database connection
    async fn open_db(&self) -> Result<IdbDatabase, JsValue> {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
        let idb_factory: IdbFactory = window
            .indexed_db()
            .map_err(to_js_error)?
            .ok_or_else(|| JsValue::from_str("IndexedDB not supported"))?;

        let open_request: IdbOpenDbRequest = idb_factory
            .open_with_u32(&self.db_name, DB_VERSION)
            .map_err(to_js_error)?;

        let result = JsFuture::from(open_request).await?;
        result
            .dyn_into::<IdbDatabase>()
            .map_err(|_| JsValue::from_str("Failed to cast to IdbDatabase"))
    }
}
