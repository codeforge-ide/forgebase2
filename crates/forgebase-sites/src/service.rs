// Service layer for sites
use crate::{deployment::DeploymentManager, domain::DomainManager, repository::*, models::*};
use forgebase_core::Result;
use sqlx::PgPool;
use std::path::PathBuf;

pub struct SitesService {
    site_repo: SiteRepository,
    deployment_repo: DeploymentRepository,
    domain_repo: DomainRepository,
    deployment_manager: DeploymentManager,
    domain_manager: DomainManager,
}

impl SitesService {
    pub fn new(pool: PgPool, storage_path: PathBuf) -> Self {
        Self {
            site_repo: SiteRepository::new(pool.clone()),
            deployment_repo: DeploymentRepository::new(pool.clone()),
            domain_repo: DomainRepository::new(pool),
            deployment_manager: DeploymentManager::new(storage_path),
            domain_manager: DomainManager::new(),
        }
    }

    // TODO: Implement service methods
}
