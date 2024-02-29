use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DbConn;
use repo::dao::repository_repo::RepositoryRepo;
use repo::dao::RepositoryRepoTrait;

use super::service::repository_service::RepositoryService;
use super::service::ServiceRepoTrait;

#[derive(Clone)]
pub struct AppState {
    pub _conn: DbConn,
    pub repo_state: RepoState,
}

#[derive(Clone)]
pub struct RepoState {
    pub repo: Arc<dyn RepositoryRepoTrait>,
    pub service: Arc<dyn ServiceRepoTrait>,
}

impl FromRef<AppState> for RepoState {
    fn from_ref(app_state: &AppState) -> RepoState {
        app_state.repo_state.clone()
    }
}

pub fn create_state(conn: DbConn) -> AppState {
    let repo: Arc<dyn RepositoryRepoTrait> = Arc::new(RepositoryRepo::new(conn.clone()));
    let service: Arc<dyn ServiceRepoTrait> = Arc::new(RepositoryService::new(Arc::clone(&repo)));

    let repo_state = RepoState { repo, service };

    AppState {
        _conn: conn,
        repo_state,
    }
}
