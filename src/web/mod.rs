use crate::config::config;
use crate::error::InternalResult;
use crate::web::state::AppState;
use axum::Router;
use utoipa::OpenApi;

mod controller;
pub mod error;
pub mod service;
pub mod state;
pub mod utils;
mod openapi;


pub async fn start_server(state: AppState) -> InternalResult<()> {
    let listener = tokio::net::TcpListener::bind(&config().SERVER.SOCKET_ADDR).await?;
    println!("Server started on socket: {}", listener.local_addr()?);
    let app = app(state.clone());
    axum::serve(listener, app).await?;
    Ok(())
}


pub fn app(state: AppState) -> Router {
    Router::new()
        .merge(controller::api_routes(state))
        .fallback(utils::api_not_found_handler)
}



// TODO finalize this API enum
enum API {
    Entity(EntityApi),
}

enum EntityApi {
    Users,
    Repos,
    UserRepoInfos,
}

impl EntityApi {
    fn to_endpoint(&self) -> &str{
        use EntityApi::*;

        match self {
            Users => "/users",
            Repos => "/repos",
            UserRepoInfos => "/user-repo-infos"
        }
    }

    fn to_tag(&self) -> &str {
        use EntityApi::*;

        match self {
            Users => "Users",
            Repos => "Repositories",
            UserRepoInfos => "User repo information"
        }
    }
}
