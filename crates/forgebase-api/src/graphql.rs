//! GraphQL API

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

/// GraphQL query root
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get API version
    async fn version(&self) -> &str {
        "1.0.0"
    }

    /// Health check
    async fn health(&self) -> bool {
        true
    }
}

/// Create GraphQL schema
pub fn create_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}
