use std::sync::Arc;

use axum::extract::FromRef;
use mongodb::Collection;
use object_store::aws::AmazonS3Builder;
use object_store::memory::InMemory;
use object_store::ObjectStore;

use collection::user::{TestUserCollection, User, UserCollection};
use collection::user_repo_info::{UserRepoInfo, UserRepoInfoCollection};
use collection::MongoCollection;
use repo::dao::repo_repository::RepoRepository;
use repo::dao::user_repo::UserRepository;
use repo::dao::user_repo_info_repository::UserRepoInfoRepository;
use repo::dao::user_repo_repository::UserRepoRepository;
use repo::dao::{
    RepoRepositoryTrait, UserRepoInfoRepositoryTrait, UserRepoRepositoryTrait, UserRepositoryTrait,
};
use repo::dto::user_repo_info_dto::CreateUserRepoInfoDto;

use crate::config::config;
use crate::error::InternalResult;
use crate::message_broker::rabbitmq::{RabbitMQOptions, RabbitMQPublisher, RabbitMQReceiver};
use crate::message_broker::{Publisher, Subscriber};
use crate::web::service::user_repo_info_receiver::UserRepoInfoReceiver;
use crate::web::service::user_repo_info_service::UserRepoInfoService;
use crate::web::service::user_repo_service::UserRepoService;
use crate::web::service::user_service::UserService;
use crate::web::service::{
    RepoServiceTrait, UserRepoInfoReceiverTrait, UserRepoInfoServiceTrait, UserRepoServiceTrait,
    UserServiceTrait,
};

use super::service::repo_service::RepositoryService;

#[derive(Clone)]
pub struct AppState {
    pub _sql_conn: Option<sea_orm::DbConn>,
    pub _nosql_conn: Option<mongodb::Database>,
    pub _rabbitmq_conn: Option<amqprs::connection::Connection>,
    pub repo_state: RepoState,
    pub user_state: UserState,
    pub user_repo_state: UserRepoState,
    pub user_repo_info_state: UserRepoInfoState,
}

impl AppState {
    pub async fn build(
        sql_conn: sea_orm::DbConn,
        nosql_conn: mongodb::Database,
        rabbitmq_conn: amqprs::connection::Connection,
    ) -> InternalResult<AppState> {
        let repo_state = RepoState::build(sql_conn.clone()).await?;

        let user_repo_info_state =
            UserRepoInfoState::build(nosql_conn.clone(), rabbitmq_conn.clone()).await?;

        let user_state = UserState::build(nosql_conn.clone(), &user_repo_info_state).await?;

        let user_repo_state =
            UserRepoState::build(&user_state, &repo_state, &user_repo_info_state).await?;

        Ok(AppState {
            _sql_conn: Some(sql_conn),
            _nosql_conn: Some(nosql_conn),
            _rabbitmq_conn: Some(rabbitmq_conn),
            repo_state,
            user_state,
            user_repo_state,
            user_repo_info_state,
        })
    }

    pub async fn build_test(
        rabbitmq_conn: amqprs::connection::Connection,
    ) -> InternalResult<AppState> {
        let sql_conn = crate::db::init_test_sql_database().await;
        let user_repo_info_state = UserRepoInfoState::build_test(rabbitmq_conn).await?;
        let repo_state = RepoState::build(sql_conn.clone()).await?;
        let user_state = UserState::build_test(&user_repo_info_state).await?;
        let user_repo_state =
            UserRepoState::build_test(&user_state, &repo_state, &user_repo_info_state).await?;

        Ok(AppState {
            _sql_conn: Some(sql_conn),
            _nosql_conn: None,
            _rabbitmq_conn: None,
            repo_state,
            user_state,
            user_repo_state,
            user_repo_info_state,
        })
    }
}

#[derive(Clone)]
pub struct RepoState {
    pub repo: Arc<dyn RepoRepositoryTrait>,
    pub service: Arc<dyn RepoServiceTrait>,
}

impl RepoState {
    async fn build(conn: sea_orm::DbConn) -> InternalResult<Self> {
        let repo: Arc<dyn RepoRepositoryTrait> = Arc::new(RepoRepository::new(conn));
        let service = Arc::new(RepositoryService::new(Arc::clone(&repo)));
        Ok(RepoState { repo, service })
    }
}

impl FromRef<AppState> for RepoState {
    fn from_ref(app_state: &AppState) -> RepoState {
        app_state.repo_state.clone()
    }
}

#[derive(Clone)]
pub struct UserState {
    pub repo: Arc<dyn UserRepositoryTrait>,
    pub service: Arc<dyn UserServiceTrait>,
}

impl UserState {
    async fn build(
        conn: mongodb::Database,
        user_repo_info_state: &UserRepoInfoState,
    ) -> InternalResult<Self> {
        let mongo_collection: Collection<User> = schema::get_collection(&conn).await?;
        let collection = Arc::new(UserCollection::new(mongo_collection));
        let repo: Arc<dyn UserRepositoryTrait> = Arc::new(UserRepository::new(collection));
        let service = Arc::new(UserService::new(
            Arc::clone(&repo),
            Arc::clone(&user_repo_info_state.service),
        ));
        Ok(UserState { service, repo })
    }

    async fn build_test(user_repo_info_state: &UserRepoInfoState) -> InternalResult<Self> {
        let collection = Arc::new(TestUserCollection::new());
        let repo: Arc<dyn UserRepositoryTrait> = Arc::new(UserRepository::new(collection));
        let service = Arc::new(UserService::new(
            Arc::clone(&repo),
            Arc::clone(&user_repo_info_state.service),
        ));
        Ok(UserState { repo, service })
    }
}

impl FromRef<AppState> for UserState {
    fn from_ref(app_state: &AppState) -> UserState {
        app_state.user_state.clone()
    }
}

#[derive(Clone)]
pub struct UserRepoState {
    pub repo: Arc<dyn UserRepoRepositoryTrait>,
    pub service: Arc<dyn UserRepoServiceTrait>,
}

impl UserRepoState {
    pub async fn build(
        user_state: &UserState,
        repo_state: &RepoState,
        user_repo_info_state: &UserRepoInfoState,
    ) -> InternalResult<Self> {
        let store = AmazonS3Builder::new()
            .with_bucket_name(&config().AWS.BUCKET_NAME)
            .with_region(&config().AWS.BUCKET_REGION)
            .with_access_key_id(&config().AWS.ACCESS_KEY)
            .with_secret_access_key(&config().AWS.SECRET_ACCESS_KEY)
            .build()?;

        Self::new(store, user_state, repo_state, user_repo_info_state)
    }

    // TODO testing
    pub async fn build_test(
        user_state: &UserState,
        repo_state: &RepoState,
        user_repo_info_state: &UserRepoInfoState,
    ) -> InternalResult<Self> {
        todo!();
        let store = InMemory::new();
        let conn = amqprs::connection::Connection::open(
            &amqprs::connection::OpenConnectionArguments::new("localhost", 5672, "guest", "guest"),
        )
        .await
        .unwrap();
        conn.register_callback(amqprs::callbacks::DefaultConnectionCallback)
            .await
            .unwrap();
        let publisher = Arc::new(
            RabbitMQPublisher::init(
                conn,
                &RabbitMQOptions {
                    queue_name: "user_repo_info",
                    durable: true,
                },
            )
            .await?,
        );

        Self::new(store, user_state, repo_state, user_repo_info_state)
    }

    fn new(
        store: impl ObjectStore,
        user_state: &UserState,
        repo_state: &RepoState,
        user_repo_info_state: &UserRepoInfoState,
    ) -> InternalResult<Self> {
        let store = Arc::new(store);
        let repo: Arc<dyn UserRepoRepositoryTrait> = Arc::new(UserRepoRepository::new(store));

        let user_service = Arc::clone(&user_state.service);
        let repo_service = Arc::clone(&repo_state.service);
        let info_publisher = Arc::clone(&user_repo_info_state.publisher);
        let service = Arc::new(UserRepoService::new(
            Arc::clone(&repo),
            user_service,
            repo_service,
            info_publisher,
        ));

        Ok(UserRepoState { repo, service })
    }
}

impl FromRef<AppState> for UserRepoState {
    fn from_ref(app_state: &AppState) -> UserRepoState {
        app_state.user_repo_state.clone()
    }
}

#[derive(Clone)]
pub struct UserRepoInfoState {
    pub repo: Arc<dyn UserRepoInfoRepositoryTrait>,
    pub service: Arc<dyn UserRepoInfoServiceTrait>,
    pub receiver: Arc<dyn UserRepoInfoReceiverTrait>,
    pub publisher: Arc<dyn Publisher<CreateUserRepoInfoDto>>,
}

impl UserRepoInfoState {
    pub async fn build(
        nosql_conn: mongodb::Database,
        rabbitmq_conn: amqprs::connection::Connection,
    ) -> InternalResult<Self> {
        let collection: Collection<UserRepoInfo> = schema::get_collection(&nosql_conn).await?;

        let collection = Arc::new(UserRepoInfoCollection { collection });

        let repo: Arc<dyn UserRepoInfoRepositoryTrait> =
            Arc::new(UserRepoInfoRepository::new(collection));

        let service = Arc::new(UserRepoInfoService::new(Arc::clone(&repo)));

        let options = RabbitMQOptions {
            queue_name: "user_repo_info",
            durable: true,
        };
        let rabbitmq_receiver =
            Arc::new(RabbitMQReceiver::init(rabbitmq_conn.clone(), &options).await?);

        let rabbitmq_publisher: Arc<dyn Publisher<CreateUserRepoInfoDto>> =
            Arc::new(RabbitMQPublisher::init(rabbitmq_conn, &options).await?);

        let receiver = Arc::new(UserRepoInfoReceiver::new(
            rabbitmq_receiver,
            service.clone(),
        ));

        Ok(UserRepoInfoState {
            repo,
            service,
            receiver,
            publisher: rabbitmq_publisher,
        })
    }

    pub async fn build_test(rabbitmq_conn: amqprs::connection::Connection) -> InternalResult<Self> {
        todo!()
        // let collection=
        //     Arc::new(TestUserRepoInfoCollection::new());
        //
        // let user_repo_info_repository: Arc<dyn UserRepoInfoRepositoryTrait> =
        //     Arc::new(UserRepoInfoRepository::new(collection));
        //
        // let user_repo_info_service = Arc::new(UserRepoInfoService::new(Arc::clone(
        //     &user_repo_info_repository,
        // )));
        //
        // let receiver = Arc::new(
        //     RabbitMQReceiver::init(
        //         rabbitmq_conn,
        //         RabbitMQOptions {
        //             queue_name: "user_repo_info",
        //             durable: true,
        //         },
        //     )
        //     .await?,
        // );
        //
        // let user_repo_info_receiver = Arc::new(UserRepoInfoReceiver::new(
        //     receiver,
        //     user_repo_info_service.clone(),
        // ));
        //
        // Ok(UserRepoInfoState {
        //     repo: user_repo_info_repository,
        //     service: user_repo_info_service,
        //     receiver: user_repo_info_receiver,
        // })
    }
}

impl FromRef<AppState> for UserRepoInfoState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_repo_info_state.clone()
    }
}
