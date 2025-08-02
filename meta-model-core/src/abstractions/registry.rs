// Abstraction Registry - Central management of format abstractions

use crate::abstractions::{AbstractionLayer, AbstractionError, AbstractionResult};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;

/// Global abstraction registry
pub static REGISTRY: Lazy<AbstractionRegistry> = Lazy::new(AbstractionRegistry::new);

/// Registry for all abstraction layers
pub struct AbstractionRegistry {
    /// Registered abstractions by format ID
    abstractions: RwLock<HashMap<String, Arc<dyn AbstractionLayer>>>,
}

impl AbstractionRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        let registry = AbstractionRegistry {
            abstractions: RwLock::new(HashMap::new()),
        };

        // Register built-in abstractions
        #[cfg(feature = "builtin-abstractions")]
        {
            registry.register_builtin_abstractions();
        }

        registry
    }

    /// Register an abstraction layer
    pub fn register(
        &self,
        abstraction: Arc<dyn AbstractionLayer>
    ) -> AbstractionResult<()> {
        let format_id = abstraction.format_id().to_string();

        let mut abstractions = self.abstractions.write()
            .map_err(|_| AbstractionError::TransformError("Failed to acquire write lock".to_string()))?;

        if abstractions.contains_key(&format_id) {
            return Err(AbstractionError::UnsupportedFormat(
                format!("Format {} already registered", format_id)
            ));
        }

        abstractions.insert(format_id, abstraction);
        Ok(())
    }

    /// Get an abstraction by format ID
    pub fn get(&self, format_id: &str) -> AbstractionResult<Arc<dyn AbstractionLayer>> {
        let abstractions = self.abstractions.read()
            .map_err(|_| AbstractionError::TransformError("Failed to acquire read lock".to_string()))?;

        abstractions.get(format_id)
            .cloned()
            .ok_or_else(|| AbstractionError::UnsupportedFormat(format_id.to_string()))
    }

    /// List all registered formats
    pub fn list_formats(&self) -> AbstractionResult<Vec<String>> {
        let abstractions = self.abstractions.read()
            .map_err(|_| AbstractionError::TransformError("Failed to acquire read lock".to_string()))?;

        Ok(abstractions.keys().cloned().collect())
    }

    /// Detect format from data
    pub async fn detect_format(&self, data: &[u8]) -> AbstractionResult<Option<String>> {
        let abstractions = self.abstractions.read()
            .map_err(|_| AbstractionError::TransformError("Failed to acquire read lock".to_string()))?;

        for (format_id, abstraction) in abstractions.iter() {
            if let Ok(report) = abstraction.validate(data).await {
                if report.valid {
                    return Ok(Some(format_id.clone()));
                }
            }
        }

        Ok(None)
    }

    /// Register built-in abstractions
    #[cfg(feature = "builtin-abstractions")]
    fn register_builtin_abstractions(&self) {
        use crate::abstractions::builtin::{
            gramps::GrampsAbstraction,
            calendar::CalendarAbstraction,
        };

        // Register GRAMPS
        let _ = self.register(Arc::new(GrampsAbstraction::new()));

        // Register calendar systems
        let _ = self.register(Arc::new(CalendarAbstraction::gregorian()));
        let _ = self.register(Arc::new(CalendarAbstraction::julian()));
    }
}

impl Default for AbstractionRegistry {
    fn default() -> Self {
        Self::new()
    }
}