//! Database schema introspection and management

use forgebase_core::{ForgeBaseError, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// Column definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub default_value: Option<String>,
    pub is_primary_key: bool,
}

/// Table definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub schema: String,
    pub columns: Vec<Column>,
}

/// Schema manager
pub struct SchemaManager {
    pool: PgPool,
}

impl SchemaManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// List all tables in the database
    pub async fn list_tables(&self) -> Result<Vec<String>> {
        let rows = sqlx::query_as::<_, (String,)>(
            r#"
            SELECT tablename 
            FROM pg_tables 
            WHERE schemaname = 'public'
            ORDER BY tablename
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|(name,)| name).collect())
    }

    /// Get table schema
    pub async fn get_table_schema(&self, table_name: &str) -> Result<Table> {
        let columns = sqlx::query_as::<_, (String, String, String, Option<String>)>(
            r#"
            SELECT 
                column_name,
                data_type,
                is_nullable,
                column_default
            FROM information_schema.columns
            WHERE table_schema = 'public' 
            AND table_name = $1
            ORDER BY ordinal_position
            "#,
        )
        .bind(table_name)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        if columns.is_empty() {
            return Err(ForgeBaseError::NotFound(format!(
                "Table '{}' not found",
                table_name
            )));
        }

        // Get primary keys
        let pk_columns = self.get_primary_keys(table_name).await?;

        let column_defs: Vec<Column> = columns
            .into_iter()
            .map(|(name, data_type, is_nullable, default_value)| Column {
                name: name.clone(),
                data_type,
                is_nullable: is_nullable == "YES",
                default_value,
                is_primary_key: pk_columns.contains(&name),
            })
            .collect();

        Ok(Table {
            name: table_name.to_string(),
            schema: "public".to_string(),
            columns: column_defs,
        })
    }

    /// Get primary key columns for a table
    async fn get_primary_keys(&self, table_name: &str) -> Result<Vec<String>> {
        let rows = sqlx::query_as::<_, (String,)>(
            r#"
            SELECT a.attname
            FROM pg_index i
            JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY(i.indkey)
            WHERE i.indrelid = $1::regclass
            AND i.indisprimary
            "#,
        )
        .bind(table_name)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|(name,)| name).collect())
    }

    /// Create a new table
    pub async fn create_table(&self, table: &Table) -> Result<()> {
        let column_defs: Vec<String> = table
            .columns
            .iter()
            .map(|col| {
                let mut def = format!("{} {}", col.name, col.data_type);
                if !col.is_nullable {
                    def.push_str(" NOT NULL");
                }
                if let Some(ref default) = col.default_value {
                    def.push_str(&format!(" DEFAULT {}", default));
                }
                if col.is_primary_key {
                    def.push_str(" PRIMARY KEY");
                }
                def
            })
            .collect();

        let create_sql = format!(
            "CREATE TABLE {} ({})",
            table.name,
            column_defs.join(", ")
        );

        sqlx::query(&create_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }

    /// Drop a table
    pub async fn drop_table(&self, table_name: &str) -> Result<()> {
        let drop_sql = format!("DROP TABLE IF EXISTS {} CASCADE", table_name);

        sqlx::query(&drop_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }

    /// Add a column to a table
    pub async fn add_column(&self, table_name: &str, column: &Column) -> Result<()> {
        let mut alter_sql = format!(
            "ALTER TABLE {} ADD COLUMN {} {}",
            table_name, column.name, column.data_type
        );

        if !column.is_nullable {
            alter_sql.push_str(" NOT NULL");
        }

        if let Some(ref default) = column.default_value {
            alter_sql.push_str(&format!(" DEFAULT {}", default));
        }

        sqlx::query(&alter_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }

    /// Drop a column from a table
    pub async fn drop_column(&self, table_name: &str, column_name: &str) -> Result<()> {
        let alter_sql = format!(
            "ALTER TABLE {} DROP COLUMN IF EXISTS {}",
            table_name, column_name
        );

        sqlx::query(&alter_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }
}
