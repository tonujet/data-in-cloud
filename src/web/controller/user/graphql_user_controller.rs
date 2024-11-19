use crate::web::state::AppState;
use crate::web::utils::validation::GraphQLValidator;
use async_graphql::{Context, Object, ResultExt};
use repo::dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};
use repo::dto::GraphqlDtoList;

use mongodb::bson::oid::ObjectId;
use repo::dto::user_repo_info_dto::UserRepoInfoDto;

#[derive(Default)]
pub struct QueryUser;

#[Object]
impl QueryUser {
    async fn get<'a>(&self, ctx: &Context<'a>, id: ObjectId) -> async_graphql::Result<UserDto> {
        let AppState {
            user_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        ctx.data_unchecked::<AppState>();
        state.service.get(&id).await.extend()
    }

    async fn list<'a>(
        &self,
        ctx: &Context<'a>,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> async_graphql::Result<GraphqlDtoList<UserDto>> {
        let AppState {
            user_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        ctx.data_unchecked::<AppState>();
        state
            .service
            .list(take, offset)
            .await
            .map(|dto_list| dto_list.into())
            .extend()
    }

    async fn list_repo_infos<'a>(
        &self,
        ctx: &Context<'a>,
        id: ObjectId,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> async_graphql::Result<GraphqlDtoList<UserRepoInfoDto>> {
        let AppState {
            user_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        ctx.data_unchecked::<AppState>();
        state
            .service
            .list_user_repos_info(id, take, offset)
            .await
            .map(|dto_list| dto_list.into())
            .extend()
    }
}

#[derive(Default)]
pub struct MutationUser;

#[Object]
impl MutationUser {
    async fn delete<'a>(&self, ctx: &Context<'a>, id: ObjectId) -> async_graphql::Result<UserDto> {
        let AppState {
            user_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        ctx.data_unchecked::<AppState>();
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
        ctx.data_unchecked::<AppState>();
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
        ctx.data_unchecked::<AppState>();
        state.service.update(&id, user_dto).await.extend()
    }
}
