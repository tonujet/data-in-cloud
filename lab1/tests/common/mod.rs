use axum_test::TestServer;
use sea_orm::EntityTrait;
use entity::prelude::Repository;
use lab1::db;

pub struct Setup {
    pub client: TestServer,
}

impl Setup {
    pub async fn clean_up(&self) {
        let conn = db::init_test_database().await;
        Repository::delete_many().exec(&conn).await.unwrap();
    }
}

impl Setup {
    pub async fn new() -> Self {
        let conn = lab1::db::init_test_database().await;
        let state = lab1::web::state::create_state(conn);
        let app = lab1::web::app(state);

        
        let instance = Self {
            client: TestServer::new(app).unwrap(),
        };
        
        instance.clean_up().await;
        
        instance
    }
}
