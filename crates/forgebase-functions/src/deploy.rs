//! Function deployment

use crate::models::*;
use forgebase_core::{ForgeBaseError, Result};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

/// Function deployer
pub struct FunctionDeployer {
    pool: PgPool,
}

impl FunctionDeployer {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Deploy a new function
    pub async fn deploy(&self, request: DeployRequest, owner_id: Uuid) -> Result<DeployResponse> {
        // Validate function code
        self.validate_code(&request.runtime, &request.code)?;

        // Compile code to WASM if needed
        let code_bytes = self.compile_code(&request.runtime, &request.code).await?;

        let function = Function {
            id: Uuid::new_v4(),
            name: request.name,
            owner_id,
            runtime: request.runtime,
            code: code_bytes,
            entry_point: request.entry_point,
            environment: request.environment.unwrap_or_default(),
            memory_limit_mb: request.memory_limit_mb.unwrap_or(128),
            timeout_seconds: request.timeout_seconds.unwrap_or(30),
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Save to database
        self.save_function(&function).await?;

        Ok(DeployResponse {
            function_id: function.id,
            url: format!("/functions/{}", function.id),
            deployed_at: function.created_at,
        })
    }

    /// Update an existing function
    pub async fn update(&self, function_id: Uuid, request: DeployRequest) -> Result<DeployResponse> {
        // Validate function code
        self.validate_code(&request.runtime, &request.code)?;

        // Compile code to WASM if needed
        let code_bytes = self.compile_code(&request.runtime, &request.code).await?;

        sqlx::query(
            r#"
            UPDATE functions 
            SET runtime = $1, code = $2, entry_point = $3, environment = $4, 
                memory_limit_mb = $5, timeout_seconds = $6, updated_at = $7
            WHERE id = $8
            "#,
        )
        .bind(format!("{:?}", request.runtime).to_lowercase())
        .bind(&code_bytes)
        .bind(&request.entry_point)
        .bind(serde_json::to_vec(&request.environment.unwrap_or_default()).unwrap())
        .bind(request.memory_limit_mb.unwrap_or(128))
        .bind(request.timeout_seconds.unwrap_or(30))
        .bind(chrono::Utc::now())
        .bind(function_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(DeployResponse {
            function_id,
            url: format!("/functions/{}", function_id),
            deployed_at: chrono::Utc::now(),
        })
    }

    /// Delete a function
    pub async fn delete(&self, function_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM functions WHERE id = $1")
            .bind(function_id)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }

    /// List functions for a user
    pub async fn list_functions(&self, owner_id: Uuid) -> Result<Vec<Function>> {
        let rows = sqlx::query_as::<_, (Uuid, String, Uuid, String, Vec<u8>, String, Vec<u8>, i32, i32, bool, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            "SELECT id, name, owner_id, runtime, code, entry_point, environment, memory_limit_mb, timeout_seconds, is_active, created_at, updated_at FROM functions WHERE owner_id = $1 ORDER BY created_at DESC"
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let runtime = match row.3.as_str() {
                    "wasm" => crate::models::FunctionRuntime::Wasm,
                    "javascript" => crate::models::FunctionRuntime::JavaScript,
                    "python" => crate::models::FunctionRuntime::Python,
                    "rust" => crate::models::FunctionRuntime::Rust,
                    _ => crate::models::FunctionRuntime::Wasm,
                };
                let environment: HashMap<String, String> =
                    serde_json::from_slice(&row.6).unwrap_or_default();
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
            .collect())
    }

    /// Validate function code
    fn validate_code(&self, _runtime: &crate::models::FunctionRuntime, _code: &str) -> Result<()> {
        // Basic validation - in production would do proper syntax checking
        Ok(())
    }

    /// Compile code to WASM
    async fn compile_code(
        &self,
        runtime: &crate::models::FunctionRuntime,
        code: &str,
    ) -> Result<Vec<u8>> {
        match runtime {
            crate::models::FunctionRuntime::Wasm => {
                // Assume code is already WASM
                base64::decode(code)
                    .map_err(|e| ForgeBaseError::Validation(format!("Invalid WASM code: {}", e)))
            }
            _ => {
                // For other runtimes, would compile to WASM
                Ok(code.as_bytes().to_vec())
            }
        }
    }

    /// Save function to database
    async fn save_function(&self, function: &Function) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO functions 
            (id, name, owner_id, runtime, code, entry_point, environment, memory_limit_mb, timeout_seconds, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(function.id)
        .bind(&function.name)
        .bind(function.owner_id)
        .bind(format!("{:?}", function.runtime).to_lowercase())
        .bind(&function.code)
        .bind(&function.entry_point)
        .bind(serde_json::to_vec(&function.environment).unwrap())
        .bind(function.memory_limit_mb)
        .bind(function.timeout_seconds)
        .bind(function.is_active)
        .bind(function.created_at)
        .bind(function.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }
}
