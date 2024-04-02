use std::sync::Arc;

use axum::extract::FromRef;
use mongodb::Collection;

use collection::user::{TestUserCollection, User, UserCollection};
use collection::MongoCollection;
use repo::dao::repository_repo::RepositoryRepo;
use repo::dao::user_repo::UserRepo;
use repo::dao::{RepositoryRepoTrait, UserRepoTrait};

use crate::error::InternalResult;
use crate::web::service::user_service::UserService;

use super::service::repository_service::RepositoryService;
use super::service::{RepoServiceTrait, UserServiceTrait};

#[derive(Clone)]
pub struct AppState {
    pub _sql_conn: Option<sea_orm::DbConn>,
    pub _nosql_conn: Option<mongodb::Database>,
    pub repo_state: RepoState,
    pub user_state: UserState,
}

#[derive(Clone)]
pub struct RepoState {
    pub repo: Arc<dyn RepositoryRepoTrait>,
    pub service: Arc<dyn RepoServiceTrait>,
}

impl RepoState {
    async fn create(conn: sea_orm::DbConn) -> InternalResult<Self> {
        let repo_repo: Arc<dyn RepositoryRepoTrait> = Arc::new(RepositoryRepo::new(conn));
        let repo_service: Arc<dyn RepoServiceTrait> =
            Arc::new(RepositoryService::new(Arc::clone(&repo_repo)));
        Ok(RepoState {
            repo: repo_repo,
            service: repo_service,
        })
    }
}

#[derive(Clone)]
pub struct UserState {
    pub repo: Arc<dyn UserRepoTrait>,
    pub service: Arc<dyn UserServiceTrait>,
}

impl UserState {
    async fn create(conn: mongodb::Database) -> InternalResult<Self> {
        let user_mongo_collection: Collection<User> = schema::get_collection(&conn).await?;
        let user_collection: Arc<dyn MongoCollection<User>> = Arc::new(UserCollection {
            collection: user_mongo_collection,
        });
        let user_repo: Arc<dyn UserRepoTrait> = Arc::new(UserRepo {
            collection: user_collection,
        });
        let user_service = Arc::new(UserService::new(Arc::clone(&user_repo)));
        Ok(UserState {
            service: user_service,
            repo: user_repo,
        })
    }

    async fn create_test() -> InternalResult<Self> {
        let user_collection: Arc<dyn MongoCollection<User>> = Arc::new(TestUserCollection::new());
        let user_repo: Arc<dyn UserRepoTrait> = Arc::new(UserRepo {
            collection: user_collection,
        });
        let user_service = Arc::new(UserService::new(Arc::clone(&user_repo)));
        Ok(UserState {
            repo: user_repo,
            service: user_service,
        })
    }
}

impl FromRef<AppState> for RepoState {
    fn from_ref(app_state: &AppState) -> RepoState {
        app_state.repo_state.clone()
    }
}

pub async fn create_state(
    sql_conn: sea_orm::DbConn,
    nosql_conn: mongodb::Database,
) -> InternalResult<AppState> {
    let repo_state = RepoState::create(sql_conn.clone()).await?;
    let user_state = UserState::create(nosql_conn.clone()).await?;
    Ok(AppState {
        _sql_conn: Some(sql_conn),
        _nosql_conn: Some(nosql_conn),
        repo_state,
        user_state,
    })
}

pub async fn create_test_state() -> InternalResult<AppState> {
    let sql_conn = crate::db::init_test_sql_database().await;
    let repo_state = RepoState::create(sql_conn.clone()).await?;
    let user_state = UserState::create_test().await?;
    Ok(AppState {
        _sql_conn: Some(sql_conn),
        _nosql_conn: None,
        repo_state,
        user_state,
    })
}
