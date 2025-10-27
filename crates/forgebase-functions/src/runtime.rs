//! WASM runtime for executing functions

use crate::models::*;
use forgebase_core::{ForgeBaseError, Result};
use std::sync::Arc;
use wasmtime::*;

/// WASM runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub memory_limit_mb: usize,
    pub max_instances: usize,
    pub enable_wasi: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            memory_limit_mb: 128,
            max_instances: 100,
            enable_wasi: true,
        }
    }
}

/// Function runtime
pub struct FunctionRuntime {
    engine: Engine,
    config: RuntimeConfig,
}

impl FunctionRuntime {
    /// Create a new function runtime
    pub fn new(config: RuntimeConfig) -> Result<Self> {
        let mut wasm_config = Config::new();
        wasm_config.wasm_multi_memory(true);
        wasm_config.wasm_memory64(false);
        wasm_config.async_support(true);

        let engine = Engine::new(&wasm_config)
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to create WASM engine: {}", e)))?;

        Ok(Self { engine, config })
    }

    /// Load a WASM module
    pub async fn load_module(&self, wasm_bytes: &[u8]) -> Result<Module> {
        Module::new(&self.engine, wasm_bytes)
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to load WASM module: {}", e)))
    }

    /// Create an execution store
    pub fn create_store(&self) -> Store<()> {
        Store::new(&self.engine, ())
    }

    /// Execute a function
    pub async fn execute(
        &self,
        module: &Module,
        function_name: &str,
        _payload: serde_json::Value,
        context: ExecutionContext,
    ) -> Result<ExecutionResult> {
        let start_time = std::time::Instant::now();
        let mut store = self.create_store();

        let memory_limit = self.config.memory_limit_mb * 1024 * 1024;
        
        // Set memory limit
        store.limiter(move |_| -> &mut dyn wasmtime::ResourceLimiter {
            Box::leak(Box::new(ResourceLimiter {
                memory_limit,
            }))
        });

        // Create instance
        let instance = Instance::new(&mut store, module, &[])
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to create instance: {}", e)))?;

        // Get the function
        let func = instance
            .get_typed_func::<(), ()>(&mut store, function_name)
            .map_err(|e| ForgeBaseError::Internal(format!("Function not found: {}", e)))?;

        // Execute with timeout
        let timeout = tokio::time::Duration::from_secs(context.timeout_seconds as u64);
        let execution = async {
            func.call_async(&mut store, ())
                .await
                .map_err(|e| ForgeBaseError::Internal(format!("Execution failed: {}", e)))
        };

        let result = tokio::time::timeout(timeout, execution).await;

        let execution_time = start_time.elapsed().as_millis() as i64;

        match result {
            Ok(Ok(_)) => Ok(ExecutionResult {
                output: serde_json::json!({"success": true}),
                logs: vec![],
                execution_time_ms: execution_time,
                memory_used_mb: 0.0,
                error: None,
            }),
            Ok(Err(e)) => Ok(ExecutionResult {
                output: serde_json::json!(null),
                logs: vec![],
                execution_time_ms: execution_time,
                memory_used_mb: 0.0,
                error: Some(e.to_string()),
            }),
            Err(_) => Ok(ExecutionResult {
                output: serde_json::json!(null),
                logs: vec![],
                execution_time_ms: execution_time,
                memory_used_mb: 0.0,
                error: Some("Execution timeout".to_string()),
            }),
        }
    }
}

/// Resource limiter for WASM instances
struct ResourceLimiter {
    memory_limit: usize,
}

impl wasmtime::ResourceLimiter for ResourceLimiter {
    fn memory_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> anyhow::Result<bool> {
        Ok(desired <= self.memory_limit)
    }

    fn table_growing(
        &mut self,
        _current: u32,
        _desired: u32,
        _maximum: Option<u32>,
    ) -> anyhow::Result<bool> {
        Ok(true)
    }
}
