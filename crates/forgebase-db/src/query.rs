//! Query builder and executor

use forgebase_core::{ForgeBaseError, Result};
use serde_json::Value;
use sqlx::{PgPool, Row, Column};
use std::collections::HashMap;

/// Query result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryResult {
    pub rows: Vec<HashMap<String, Value>>,
    pub rows_affected: u64,
}

/// Query builder
pub struct QueryBuilder {
    pool: PgPool,
}

impl QueryBuilder {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Execute a raw SQL query
    pub async fn execute_raw(&self, sql: &str) -> Result<QueryResult> {
        let rows = sqlx::query(sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        let mut result_rows = Vec::new();
        for row in rows {
            let mut row_map = HashMap::new();
            for (i, column) in row.columns().iter().enumerate() {
                let col_name = column.name().to_string();
                let value: Option<String> = row.try_get(i).ok();
                row_map.insert(
                    col_name,
                    value.map(|v| Value::String(v)).unwrap_or(Value::Null),
                );
            }
            result_rows.push(row_map);
        }

        let rows_affected = result_rows.len() as u64;
        Ok(QueryResult {
            rows: result_rows,
            rows_affected,
        })
    }

    /// Execute a SELECT query
    pub async fn select(
        &self,
        table: &str,
        columns: &[&str],
        filters: Option<HashMap<String, Value>>,
    ) -> Result<QueryResult> {
        let cols = if columns.is_empty() {
            "*".to_string()
        } else {
            columns.join(", ")
        };

        let mut query = format!("SELECT {} FROM {}", cols, table);

        if let Some(filters) = filters {
            if !filters.is_empty() {
                let conditions: Vec<String> = filters
                    .iter()
                    .map(|(k, v)| format!("{} = '{}'", k, v))
                    .collect();
                query.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
            }
        }

        self.execute_raw(&query).await
    }

    /// Execute an INSERT query
    pub async fn insert(
        &self,
        table: &str,
        data: HashMap<String, Value>,
    ) -> Result<QueryResult> {
        let columns: Vec<String> = data.keys().cloned().collect();
        let values: Vec<String> = data
            .values()
            .map(|v| match v {
                Value::String(s) => format!("'{}'", s),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "NULL".to_string(),
                _ => format!("'{}'", v.to_string()),
            })
            .collect();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            table,
            columns.join(", "),
            values.join(", ")
        );

        self.execute_raw(&query).await
    }

    /// Execute an UPDATE query
    pub async fn update(
        &self,
        table: &str,
        data: HashMap<String, Value>,
        filters: HashMap<String, Value>,
    ) -> Result<QueryResult> {
        let set_clauses: Vec<String> = data
            .iter()
            .map(|(k, v)| match v {
                Value::String(s) => format!("{} = '{}'", k, s),
                Value::Number(n) => format!("{} = {}", k, n),
                Value::Bool(b) => format!("{} = {}", k, b),
                Value::Null => format!("{} = NULL", k),
                _ => format!("{} = '{}'", k, v.to_string()),
            })
            .collect();

        let where_clauses: Vec<String> = filters
            .iter()
            .map(|(k, v)| format!("{} = '{}'", k, v))
            .collect();

        let query = format!(
            "UPDATE {} SET {} WHERE {} RETURNING *",
            table,
            set_clauses.join(", "),
            where_clauses.join(" AND ")
        );

        self.execute_raw(&query).await
    }

    /// Execute a DELETE query
    pub async fn delete(
        &self,
        table: &str,
        filters: HashMap<String, Value>,
    ) -> Result<QueryResult> {
        let where_clauses: Vec<String> = filters
            .iter()
            .map(|(k, v)| format!("{} = '{}'", k, v))
            .collect();

        let query = format!(
            "DELETE FROM {} WHERE {}",
            table,
            where_clauses.join(" AND ")
        );

        let result = sqlx::query(&query)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(QueryResult {
            rows: vec![],
            rows_affected: result.rows_affected(),
        })
    }
}
