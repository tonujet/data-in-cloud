use std::sync::Arc;

use axum::extract::FromRef;
use mongodb::Collection;
use object_store::aws::AmazonS3Builder;
use object_store::memory::InMemory;
use object_store::ObjectStore;

use collection::user::{TestUserCollection, User, UserCollection};
use collection::MongoCollection;
use repo::dao::repository_repo::RepositoryRepo;
use repo::dao::user_repo::UserRepo;
use repo::dao::user_repository_repo::UserRepositoryRepo;
use repo::dao::{RepositoryRepoTrait, UserRepoTrait, UserRepositoryRepoTrait};
use crate::config::config;

use crate::error::InternalResult;
use crate::web::service::user_repo_service::UserRepoService;
use crate::web::service::user_service::UserService;

use super::service::repo_service::RepositoryService;
use super::service::{RepoServiceTrait, UserRepoServiceTrait, UserServiceTrait};

#[derive(Clone)]
pub struct AppState {
    pub _sql_conn: Option<sea_orm::DbConn>,
    pub _nosql_conn: Option<mongodb::Database>,
    pub repo_state: RepoState,
    pub user_state: UserState,
    pub user_repo_state: UserRepoState,
}

impl AppState {
    pub async fn build(
        sql_conn: sea_orm::DbConn,
        nosql_conn: mongodb::Database,
    ) -> InternalResult<AppState> {
        let repo_state = RepoState::build(sql_conn.clone()).await?;
        let user_state = UserState::build(nosql_conn.clone()).await?;
        let user_repo_state = UserRepoState::build(&user_state, &repo_state)?;
        Ok(AppState {
            _sql_conn: Some(sql_conn),
            _nosql_conn: Some(nosql_conn),
            repo_state,
            user_state,
            user_repo_state,
        })
    }

    pub async fn build_test() -> InternalResult<AppState> {
        let sql_conn = crate::db::init_test_sql_database().await;
        let repo_state = RepoState::build(sql_conn.clone()).await?;
        let user_state = UserState::build_test().await?;
        let user_repo_state = UserRepoState::build_test(&user_state, &repo_state)?;
        Ok(AppState {
            _sql_conn: Some(sql_conn),
            _nosql_conn: None,
            repo_state,
            user_state,
            user_repo_state,
        })
    }
}

#[derive(Clone)]
pub struct RepoState {
    pub repo: Arc<dyn RepositoryRepoTrait>,
    pub service: Arc<dyn RepoServiceTrait>,
}

impl RepoState {
    async fn build(conn: sea_orm::DbConn) -> InternalResult<Self> {
        let repo_repo: Arc<dyn RepositoryRepoTrait> = Arc::new(RepositoryRepo::new(conn));
        let repo_service: Arc<dyn RepoServiceTrait> =
            Arc::new(RepositoryService::new(Arc::clone(&repo_repo)));
        Ok(RepoState {
            repo: repo_repo,
            service: repo_service,
        })
    }
}

impl FromRef<AppState> for RepoState {
    fn from_ref(app_state: &AppState) -> RepoState {
        app_state.repo_state.clone()
    }
}

#[derive(Clone)]
pub struct UserState {
    pub repo: Arc<dyn UserRepoTrait>,
    pub service: Arc<dyn UserServiceTrait>,
}

impl UserState {
    async fn build(conn: mongodb::Database) -> InternalResult<Self> {
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

    async fn build_test() -> InternalResult<Self> {
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

impl FromRef<AppState> for UserState {
    fn from_ref(app_state: &AppState) -> UserState {
        app_state.user_state.clone()
    }
}

#[derive(Clone)]
pub struct UserRepoState {
    pub repo: Arc<dyn UserRepositoryRepoTrait>,
    pub service: Arc<dyn UserRepoServiceTrait>,
}

impl UserRepoState {
    pub fn build(user_state: &UserState, repo_state: &RepoState) -> InternalResult<Self> {
        let store = AmazonS3Builder::new()
            .with_bucket_name(&config().AWS.BUCKET_NAME)
            .with_region(&config().AWS.BUCKET_REGION)
            .with_access_key_id(&config().AWS.ACCESS_KEY)
            .with_secret_access_key(&config().AWS.SECRET_ACCESS_KEY)
            .build()?;
        Self::new(store, user_state, repo_state)
    }

    pub fn build_test(user_state: &UserState, repo_state: &RepoState) -> InternalResult<Self> {
        let store = InMemory::new();
        Self::new(store, user_state, repo_state)
    }

    fn new(store: impl ObjectStore, user_state: &UserState, repo_state: &RepoState) -> InternalResult<Self>{
        let store = Arc::new(store);
        let user_repository_repo: Arc<dyn UserRepositoryRepoTrait> =
            Arc::new(UserRepositoryRepo::new(store));

        let user_service = Arc::clone(&user_state.service);
        let repo_service = Arc::clone(&repo_state.service);
        let user_repo_service = Arc::new(UserRepoService::new(
            Arc::clone(&user_repository_repo),
            user_service,
            repo_service,
        ));

        Ok(UserRepoState {
            repo: user_repository_repo,
            service: user_repo_service,
        })
    }
}

impl FromRef<AppState> for UserRepoState {
    fn from_ref(app_state: &AppState) -> UserRepoState {
        app_state.user_repo_state.clone()
    }
}
