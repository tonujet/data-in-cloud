use crate::web::state::AppState;
use async_graphql::{Context, Object, ResultExt};
use dto::repo_dto::RepoDto;
use dto::user_dto::UserDto;
use dto::{OneToManyDto, OneToOneDto};
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;

#[derive(Default)]
pub struct UserRepoQuery;

#[Object]
impl UserRepoQuery {
    async fn list_repos<'a>(
        &self,
        ctx: &Context<'a>,
        user_id: ObjectId,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> async_graphql::Result<OneToManyDto<UserDto, RepoDto>> {
        let AppState {
            user_repo_state: state,
            ..
        } = ctx.data_unchecked::<AppState>();
        state
            .service
            .list_pairs(&user_id, take, offset)
            .await
            .map(|user_repos| user_repos.into())
            .extend()
    }
}

#[derive(Default)]
pub struct UserRepoMutation;

#[Object]
impl UserRepoMutation {
    async fn delete_repo<'a>(
        &self,
        ctx: &Context<'a>,
        user_id: ObjectId,
        repo_id: Uuid,
    ) -> async_graphql::Result<OneToOneDto<UserDto, RepoDto>> {
        let AppState {
            user_repo_state: state,
            ..
        } = ctx.data_unchecked::<AppState>();
        state.service.delete_pair(&user_id, &repo_id).await.extend()
    }

    async fn add_repo<'a>(
        &self,
        ctx: &Context<'a>,
        user_id: ObjectId,
        repo_id: Uuid,
    ) -> async_graphql::Result<OneToOneDto<UserDto, RepoDto>> {
        let AppState {
            user_repo_state: state,
            ..
        } = ctx.data_unchecked::<AppState>();
        state.service.add_pair(&user_id, &repo_id).await.extend()
    }
}
