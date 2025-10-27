//! Database initialization utilities

use crate::migrations::{Migration, MigrationManager};
use forgebase_core::Result;
use sqlx::PgPool;

/// Initialize database with migrations
pub async fn init_database(pool: &PgPool) -> Result<()> {
    let manager = MigrationManager::new(pool.clone());

    // Define migrations
    let migrations = vec![
        Migration {
            version: 1,
            name: "create_auth_tables".to_string(),
            up_sql: include_str!("../../../migrations/001_create_auth_tables.sql").to_string(),
            down_sql: "-- Not implemented".to_string(),
        },
        Migration {
            version: 2,
            name: "create_sites_tables".to_string(),
            up_sql: include_str!("../../../migrations/002_create_sites_tables.sql").to_string(),
            down_sql: "-- Not implemented".to_string(),
        },
    ];

    // Run migrations
    let applied = manager.migrate(&migrations).await?;
    
    if !applied.is_empty() {
        tracing::info!("Applied {} migrations", applied.len());
        for version in applied {
            tracing::info!("  - Migration {}", version);
        }
    } else {
        tracing::info!("No new migrations to apply");
    }

    Ok(())
}
