use crate::web::state::AppState;
use crate::web::utils::validation::GraphQLValidator;
use async_graphql::{Context, MergedObject, Object, ResultExt};
use dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};

use crate::web::controller::user_repo::graphql_user_repo_controller::{
    UserRepoMutation, UserRepoQuery,
};
use dto::user_repo_info_dto::UserRepoInfoDto;
use dto::DtoList;
use mongodb::bson::oid::ObjectId;

#[derive(MergedObject, Default)]
pub struct QueryUser(QueryUserToMerge, UserRepoQuery);

#[derive(MergedObject, Default)]
pub struct MutationUser(MutationUserToMerge, UserRepoMutation);

#[derive(Default)]
struct QueryUserToMerge;

#[Object]
impl QueryUserToMerge {
    async fn get<'a>(&self, ctx: &Context<'a>, id: ObjectId) -> async_graphql::Result<UserDto> {
        let AppState {
            user_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        state.service.get(&id).await.extend()
    }

    async fn list<'a>(
        &self,
        ctx: &Context<'a>,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> async_graphql::Result<DtoList<UserDto>> {
        let AppState {
            user_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        state.service.list(take, offset).await.extend()
    }

    async fn list_repo_infos<'a>(
        &self,
        ctx: &Context<'a>,
        id: ObjectId,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> async_graphql::Result<DtoList<UserRepoInfoDto>> {
        let AppState {
            user_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        state
            .service
            .list_user_repos_info(id, take, offset)
            .await
            .extend()
    }
}

#[derive(Default)]
struct MutationUserToMerge;

#[Object]
impl MutationUserToMerge {
    async fn delete<'a>(&self, ctx: &Context<'a>, id: ObjectId) -> async_graphql::Result<UserDto> {
        let AppState {
            user_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        state.service.delete(&id).await.extend()
    }

    async fn create<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "GraphQLValidator::default()"))] user_dto: CreateUserDto,
    ) -> async_graphql::Result<UserDto> {
        let AppState {
            user_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        state.service.create(user_dto).await.extend()
    }

    async fn update<'a>(
        &self,
        ctx: &Context<'a>,
        id: ObjectId,
        #[graphql(validator(custom = "GraphQLValidator::default()"))] user_dto: UpdateUserDto,
    ) -> async_graphql::Result<UserDto> {
        let AppState {
            user_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        state.service.update(&id, user_dto).await.extend()
    }
}
