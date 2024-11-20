use crate::web::state::AppState;
use async_graphql::{Context, Object, ResultExt};

use mongodb::bson::oid::ObjectId;
use dto::DtoList;
use dto::user_repo_info_dto::UserRepoInfoDto;

#[derive(Default)]
pub struct QueryUserRepoInfo;

#[Object]
impl QueryUserRepoInfo {
    async fn get<'a>(
        &self,
        ctx: &Context<'a>,
        id: ObjectId,
    ) -> async_graphql::Result<UserRepoInfoDto> {
        let AppState {
            user_repo_info_state: state,
            ..
        } = ctx.data_unchecked::<AppState>();
        state.service.get(&id).await.extend()
    }

    async fn list<'a>(
        &self,
        ctx: &Context<'a>,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> async_graphql::Result<DtoList<UserRepoInfoDto>> {
        let AppState {
            user_repo_info_state: state,
            ..
        } = ctx.data_unchecked::<AppState>();
        state
            .service
            .list(take, offset)
            .await
            .extend()
    }
}
