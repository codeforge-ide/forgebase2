//! Function executor

use crate::models::*;
use crate::runtime::{FunctionRuntime as WasmRuntime, RuntimeConfig};
use forgebase_core::{ForgeBaseError, Result};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Function executor
pub struct FunctionExecutor {
    pool: PgPool,
    runtime: Arc<WasmRuntime>,
}

impl FunctionExecutor {
    /// Create a new function executor
    pub fn new(pool: PgPool, config: RuntimeConfig) -> Result<Self> {
        let runtime = Arc::new(WasmRuntime::new(config)?);
        Ok(Self { pool, runtime })
    }

    /// Execute a function
    pub async fn execute(&self, request: InvocationRequest) -> Result<InvocationResponse> {
        let start_time = std::time::Instant::now();

        // Load function from database
        let function = self.get_function(request.function_id).await?;

        if !function.is_active {
            return Err(ForgeBaseError::Validation(
                "Function is not active".to_string(),
            ));
        }

        // Create execution context
        let context = ExecutionContext {
            function_id: function.id,
            request_id: Uuid::new_v4(),
            environment: function.environment.clone(),
            memory_limit_mb: function.memory_limit_mb,
            timeout_seconds: function.timeout_seconds,
        };

        // Execute based on runtime type
        let result = match function.runtime {
            crate::models::FunctionRuntime::Wasm => {
                self.execute_wasm(&function, request.payload, context).await?
            }
            crate::models::FunctionRuntime::JavaScript => {
                self.execute_javascript(&function, request.payload, context)
                    .await?
            }
            _ => {
                return Err(ForgeBaseError::Internal(
                    "Runtime not yet implemented".to_string(),
                ))
            }
        };

        let execution_time = start_time.elapsed().as_millis() as i64;

        // Record invocation
        self.record_invocation(&function, &result, execution_time)
            .await?;

        Ok(InvocationResponse {
            status_code: if result.error.is_none() { 200 } else { 500 },
            body: result.output,
            headers: HashMap::new(),
            execution_time_ms: execution_time,
            memory_used_mb: result.memory_used_mb,
        })
    }

    /// Execute WASM function
    async fn execute_wasm(
        &self,
        function: &Function,
        payload: serde_json::Value,
        context: ExecutionContext,
    ) -> Result<ExecutionResult> {
        let module = self.runtime.load_module(&function.code).await?;
        self.runtime
            .execute(&module, &function.entry_point, payload, context)
            .await
    }

    /// Execute JavaScript function
    async fn execute_javascript(
        &self,
        _function: &Function,
        _payload: serde_json::Value,
        _context: ExecutionContext,
    ) -> Result<ExecutionResult> {
        // Placeholder for JavaScript execution
        // Would use a JS runtime like deno_core or boa
        Ok(ExecutionResult {
            output: serde_json::json!({"message": "JavaScript runtime not yet implemented"}),
            logs: vec![],
            execution_time_ms: 0,
            memory_used_mb: 0.0,
            error: None,
        })
    }

    /// Get function by ID
    async fn get_function(&self, function_id: Uuid) -> Result<Function> {
        sqlx::query_as::<_, (Uuid, String, Uuid, String, Vec<u8>, String, Vec<u8>, i32, i32, bool, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            "SELECT id, name, owner_id, runtime, code, entry_point, environment, memory_limit_mb, timeout_seconds, is_active, created_at, updated_at FROM functions WHERE id = $1"
        )
        .bind(function_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?
        .map(|row| {
            let runtime = match row.3.as_str() {
                "wasm" => crate::models::FunctionRuntime::Wasm,
                "javascript" => crate::models::FunctionRuntime::JavaScript,
                "python" => crate::models::FunctionRuntime::Python,
                "rust" => crate::models::FunctionRuntime::Rust,
                _ => crate::models::FunctionRuntime::Wasm,
            };
            let environment: HashMap<String, String> = serde_json::from_slice(&row.6).unwrap_or_default();
            Function {
                id: row.0,
                name: row.1,
                owner_id: row.2,
                runtime,
                code: row.4,
                entry_point: row.5,
                environment,
                memory_limit_mb: row.7,
                timeout_seconds: row.8,
                is_active: row.9,
                created_at: row.10,
                updated_at: row.11,
            }
        })
        .ok_or_else(|| ForgeBaseError::NotFound("Function not found".to_string()))
    }

    /// Record function invocation
    async fn record_invocation(
        &self,
        function: &Function,
        result: &ExecutionResult,
        execution_time_ms: i64,
    ) -> Result<()> {
        let success = result.error.is_none();

        sqlx::query(
            r#"
            INSERT INTO function_invocations 
            (id, function_id, success, execution_time_ms, memory_used_mb, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(function.id)
        .bind(success)
        .bind(execution_time_ms)
        .bind(result.memory_used_mb)
        .bind(chrono::Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }
}
