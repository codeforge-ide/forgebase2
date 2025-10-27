//! Function runtime models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub runtime: FunctionRuntime,
    pub code: Vec<u8>,
    pub entry_point: String,
    pub environment: HashMap<String, String>,
    pub memory_limit_mb: i32,
    pub timeout_seconds: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Function runtime type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionRuntime {
    Wasm,
    JavaScript,
    Python,
    Rust,
}

/// Function invocation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationRequest {
    pub function_id: Uuid,
    pub payload: serde_json::Value,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
}

/// Function invocation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationResponse {
    pub status_code: u16,
    pub body: serde_json::Value,
    pub headers: HashMap<String, String>,
    pub execution_time_ms: i64,
    pub memory_used_mb: f64,
}

/// Function deployment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployRequest {
    pub name: String,
    pub runtime: FunctionRuntime,
    pub code: String,
    pub entry_point: String,
    pub environment: Option<HashMap<String, String>>,
    pub memory_limit_mb: Option<i32>,
    pub timeout_seconds: Option<i32>,
}

/// Function deployment response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployResponse {
    pub function_id: Uuid,
    pub url: String,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}

/// Function execution context
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub function_id: Uuid,
    pub request_id: Uuid,
    pub environment: HashMap<String, String>,
    pub memory_limit_mb: i32,
    pub timeout_seconds: i32,
}

/// Function execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub output: serde_json::Value,
    pub logs: Vec<String>,
    pub execution_time_ms: i64,
    pub memory_used_mb: f64,
    pub error: Option<String>,
}

/// Function statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionStats {
    pub function_id: Uuid,
    pub total_invocations: i64,
    pub successful_invocations: i64,
    pub failed_invocations: i64,
    pub avg_execution_time_ms: f64,
    pub avg_memory_used_mb: f64,
    pub last_invoked_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Function log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionLog {
    pub id: Uuid,
    pub function_id: Uuid,
    pub request_id: Uuid,
    pub level: LogLevel,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Log level
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
