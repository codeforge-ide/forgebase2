//! Real-time database subscriptions using PostgreSQL LISTEN/NOTIFY

use forgebase_core::{ForgeBaseError, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::broadcast;
use uuid::Uuid;

/// Database change event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    Insert,
    Update,
    Delete,
}

/// Database change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEvent {
    pub id: Uuid,
    pub table: String,
    pub change_type: ChangeType,
    pub old_record: Option<serde_json::Value>,
    pub new_record: Option<serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Real-time subscription manager
pub struct RealtimeManager {
    pool: PgPool,
    sender: broadcast::Sender<ChangeEvent>,
}

impl RealtimeManager {
    /// Create a new realtime manager
    pub fn new(pool: PgPool) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self { pool, sender }
    }

    /// Subscribe to table changes
    pub fn subscribe(&self, table: Option<String>) -> RealtimeSubscription {
        let receiver = self.sender.subscribe();
        RealtimeSubscription { receiver, table }
    }

    /// Publish a change event
    pub async fn publish(&self, event: ChangeEvent) -> Result<()> {
        self.sender
            .send(event)
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to publish event: {}", e)))?;
        Ok(())
    }

    /// Start listening to PostgreSQL notifications
    pub async fn start_listener(&self) -> Result<()> {
        // This would use PostgreSQL LISTEN/NOTIFY in production
        // For now, this is a placeholder structure
        Ok(())
    }

    /// Create triggers for a table to enable real-time updates
    pub async fn enable_realtime_for_table(&self, table: &str) -> Result<()> {
        let trigger_fn = format!(
            r#"
            CREATE OR REPLACE FUNCTION notify_{table}_changes()
            RETURNS TRIGGER AS $$
            DECLARE
                payload json;
            BEGIN
                IF TG_OP = 'DELETE' THEN
                    payload = json_build_object(
                        'table', TG_TABLE_NAME,
                        'type', 'delete',
                        'old', row_to_json(OLD)
                    );
                    PERFORM pg_notify('forgebase_changes', payload::text);
                    RETURN OLD;
                ELSIF TG_OP = 'UPDATE' THEN
                    payload = json_build_object(
                        'table', TG_TABLE_NAME,
                        'type', 'update',
                        'old', row_to_json(OLD),
                        'new', row_to_json(NEW)
                    );
                    PERFORM pg_notify('forgebase_changes', payload::text);
                    RETURN NEW;
                ELSIF TG_OP = 'INSERT' THEN
                    payload = json_build_object(
                        'table', TG_TABLE_NAME,
                        'type', 'insert',
                        'new', row_to_json(NEW)
                    );
                    PERFORM pg_notify('forgebase_changes', payload::text);
                    RETURN NEW;
                END IF;
            END;
            $$ LANGUAGE plpgsql;

            CREATE TRIGGER {table}_changes_trigger
            AFTER INSERT OR UPDATE OR DELETE ON {table}
            FOR EACH ROW EXECUTE FUNCTION notify_{table}_changes();
            "#,
            table = table
        );

        sqlx::query(&trigger_fn)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }

    /// Disable real-time updates for a table
    pub async fn disable_realtime_for_table(&self, table: &str) -> Result<()> {
        let drop_trigger = format!(
            "DROP TRIGGER IF EXISTS {}_changes_trigger ON {}",
            table, table
        );

        sqlx::query(&drop_trigger)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }
}

/// Real-time subscription handle
pub struct RealtimeSubscription {
    receiver: broadcast::Receiver<ChangeEvent>,
    table: Option<String>,
}

impl RealtimeSubscription {
    /// Receive the next change event
    pub async fn recv(&mut self) -> Result<ChangeEvent> {
        loop {
            let event = self
                .receiver
                .recv()
                .await
                .map_err(|e| ForgeBaseError::Internal(format!("Subscription error: {}", e)))?;

            // Filter by table if specified
            if let Some(ref table) = self.table {
                if &event.table == table {
                    return Ok(event);
                }
            } else {
                return Ok(event);
            }
        }
    }
}

impl Clone for RealtimeManager {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            sender: self.sender.clone(),
        }
    }
}
