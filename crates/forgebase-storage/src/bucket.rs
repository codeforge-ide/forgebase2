//! Bucket management

use forgebase_core::{ForgeBaseError, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// Bucket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub is_public: bool,
    pub max_file_size: Option<i64>,
    pub allowed_mime_types: Option<Vec<String>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Bucket manager
pub struct BucketManager {
    pool: PgPool,
}

impl BucketManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new bucket
    pub async fn create_bucket(&self, bucket: &Bucket) -> Result<Bucket> {
        let result = sqlx::query_as::<_, (Uuid, String, Uuid, bool, Option<i64>, Option<Vec<String>>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            r#"
            INSERT INTO storage_buckets (id, name, owner_id, is_public, max_file_size, allowed_mime_types, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, name, owner_id, is_public, max_file_size, allowed_mime_types, created_at, updated_at
            "#,
        )
        .bind(bucket.id)
        .bind(&bucket.name)
        .bind(bucket.owner_id)
        .bind(bucket.is_public)
        .bind(bucket.max_file_size)
        .bind(&bucket.allowed_mime_types)
        .bind(bucket.created_at)
        .bind(bucket.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(Bucket {
            id: result.0,
            name: result.1,
            owner_id: result.2,
            is_public: result.3,
            max_file_size: result.4,
            allowed_mime_types: result.5,
            created_at: result.6,
            updated_at: result.7,
        })
    }

    /// Get a bucket by name
    pub async fn get_bucket(&self, name: &str) -> Result<Option<Bucket>> {
        let result = sqlx::query_as::<_, (Uuid, String, Uuid, bool, Option<i64>, Option<Vec<String>>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            "SELECT id, name, owner_id, is_public, max_file_size, allowed_mime_types, created_at, updated_at FROM storage_buckets WHERE name = $1",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(result.map(|r| Bucket {
            id: r.0,
            name: r.1,
            owner_id: r.2,
            is_public: r.3,
            max_file_size: r.4,
            allowed_mime_types: r.5,
            created_at: r.6,
            updated_at: r.7,
        }))
    }

    /// List buckets for a user
    pub async fn list_buckets(&self, owner_id: Uuid) -> Result<Vec<Bucket>> {
        let results = sqlx::query_as::<_, (Uuid, String, Uuid, bool, Option<i64>, Option<Vec<String>>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            "SELECT id, name, owner_id, is_public, max_file_size, allowed_mime_types, created_at, updated_at FROM storage_buckets WHERE owner_id = $1 ORDER BY created_at DESC",
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(results
            .into_iter()
            .map(|r| Bucket {
                id: r.0,
                name: r.1,
                owner_id: r.2,
                is_public: r.3,
                max_file_size: r.4,
                allowed_mime_types: r.5,
                created_at: r.6,
                updated_at: r.7,
            })
            .collect())
    }

    /// Update a bucket
    pub async fn update_bucket(&self, bucket: &Bucket) -> Result<Bucket> {
        let result = sqlx::query_as::<_, (Uuid, String, Uuid, bool, Option<i64>, Option<Vec<String>>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            r#"
            UPDATE storage_buckets 
            SET is_public = $1, max_file_size = $2, allowed_mime_types = $3, updated_at = $4
            WHERE id = $5
            RETURNING id, name, owner_id, is_public, max_file_size, allowed_mime_types, created_at, updated_at
            "#,
        )
        .bind(bucket.is_public)
        .bind(bucket.max_file_size)
        .bind(&bucket.allowed_mime_types)
        .bind(chrono::Utc::now())
        .bind(bucket.id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(Bucket {
            id: result.0,
            name: result.1,
            owner_id: result.2,
            is_public: result.3,
            max_file_size: result.4,
            allowed_mime_types: result.5,
            created_at: result.6,
            updated_at: result.7,
        })
    }

    /// Delete a bucket
    pub async fn delete_bucket(&self, bucket_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM storage_buckets WHERE id = $1")
            .bind(bucket_id)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }
}
