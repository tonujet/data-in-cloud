use crate::web::state::AppState;
use crate::web::utils::validation::GraphQLValidator;
use async_graphql::{Context, Object, ResultExt};
use repo::dto::repo_dto::{CreateUpdateRepoDto, RepoDto};
use uuid::Uuid;
use repo::dto::DtoList;

#[derive(Default)]
pub struct QueryRepo;

#[Object]
impl QueryRepo {
    async fn get<'a>(&self, ctx: &Context<'a>, id: Uuid) -> async_graphql::Result<RepoDto> {
        let AppState {
            repo_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        ctx.data_unchecked::<AppState>();
        state.service.get(&id).await.extend()
    }

    async fn list<'a>(
        &self,
        ctx: &Context<'a>,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> async_graphql::Result<DtoList<RepoDto>> {
        let AppState {
            repo_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        ctx.data_unchecked::<AppState>();
        state
            .service
            .list(take, offset)
            .await
            .extend()
    }
}

#[derive(Default)]
pub struct MutationRepo;

#[Object]
impl MutationRepo {
    async fn delete<'a>(&self, ctx: &Context<'a>, id: Uuid) -> async_graphql::Result<RepoDto> {
        let AppState {
            repo_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        ctx.data_unchecked::<AppState>();
        state.service.delete(&id).await.extend()
    }

    async fn create<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "GraphQLValidator::default()"))] repo_dto: CreateUpdateRepoDto,
    ) -> async_graphql::Result<RepoDto> {
        let AppState {
            repo_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        ctx.data_unchecked::<AppState>();
        state.service.create(repo_dto).await.extend()
    }

    async fn update<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        #[graphql(validator(custom = "GraphQLValidator::default()"))] repo_dto: CreateUpdateRepoDto,
    ) -> async_graphql::Result<RepoDto> {
        let AppState {
            repo_state: state, ..
        } = ctx.data_unchecked::<AppState>();
        ctx.data_unchecked::<AppState>();
        state.service.update(&id, repo_dto).await.extend()
    }
}
