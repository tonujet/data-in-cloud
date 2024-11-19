use axum_test::TestServer;
use sea_orm::EntityTrait;

use entity::prelude::Repository;

pub struct Setup {
    pub client: TestServer,
}

impl Setup {
    pub async fn clean_up(&self) {
        let sql_conn = ia_11_vorobei_ant::db::init_test_sql_database().await;
        Repository::delete_many().exec(&sql_conn).await.unwrap();
    }
}

impl Setup {
    pub async fn new() -> Self {
        let state = ia_11_vorobei_ant::web::state::AppState::build_test()
            .await
            .unwrap();
        let app = ia_11_vorobei_ant::web::app(state);

        let instance = Self {
            client: TestServer::new(app).unwrap(),
        };

        instance.clean_up().await;
        instance
    }
}