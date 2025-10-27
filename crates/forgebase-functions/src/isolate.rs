//! WASM isolate management for secure execution

use forgebase_core::{ForgeBaseError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use wasmtime::Module;

/// Isolate pool for managing WASM instances
pub struct IsolatePool {
    modules: Arc<RwLock<HashMap<Uuid, Module>>>,
    max_size: usize,
}

impl IsolatePool {
    /// Create a new isolate pool
    pub fn new(max_size: usize) -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new())),
            max_size,
        }
    }

    /// Get or load a module
    pub async fn get_or_load(&self, function_id: Uuid, module: Module) -> Result<Module> {
        let mut modules = self.modules.write().await;

        if let Some(cached_module) = modules.get(&function_id) {
            return Ok(cached_module.clone());
        }

        if modules.len() >= self.max_size {
            // Simple eviction - remove oldest
            if let Some(key) = modules.keys().next().cloned() {
                modules.remove(&key);
            }
        }

        modules.insert(function_id, module.clone());
        Ok(module)
    }

    /// Remove a module from the pool
    pub async fn remove(&self, function_id: Uuid) -> Result<()> {
        let mut modules = self.modules.write().await;
        modules.remove(&function_id);
        Ok(())
    }

    /// Clear the pool
    pub async fn clear(&self) -> Result<()> {
        let mut modules = self.modules.write().await;
        modules.clear();
        Ok(())
    }

    /// Get pool size
    pub async fn size(&self) -> usize {
        let modules = self.modules.read().await;
        modules.len()
    }
}

impl Clone for IsolatePool {
    fn clone(&self) -> Self {
        Self {
            modules: Arc::clone(&self.modules),
            max_size: self.max_size,
        }
    }
}
