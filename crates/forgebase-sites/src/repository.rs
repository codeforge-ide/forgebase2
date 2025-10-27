// Repository placeholder for sites
use crate::models::*;
use forgebase_core::{ForgeBaseError, Result};
use sqlx::PgPool;
use uuid::Uuid;

pub struct SiteRepository {
    pool: PgPool,
}

impl SiteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, site: &Site) -> Result<Site> {
        // TODO: Implement
        unimplemented!()
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Site>> {
        // TODO: Implement
        unimplemented!()
    }
}

pub struct DeploymentRepository {
    pool: PgPool,
}

impl DeploymentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, deployment: &Deployment) -> Result<Deployment> {
        // TODO: Implement
        unimplemented!()
    }
}

pub struct DomainRepository {
    pool: PgPool,
}

impl DomainRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, domain: &Domain) -> Result<Domain> {
        // TODO: Implement
        unimplemented!()
    }
}
