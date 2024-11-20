use repo_dto::RepoDto;
use serde::{Deserialize, Serialize};
use user_dto::UserDto;
use user_repo_info_dto::UserRepoInfoDto;

pub mod repo_dto;
pub mod user_dto;
pub mod user_repo_info_dto;
pub mod utils;

#[derive(
    Serialize, Deserialize, PartialEq, Debug, async_graphql::SimpleObject, utoipa::ToSchema,
)]
#[graphql(concrete(name = "RepoDtoList", params(RepoDto)))]
#[graphql(concrete(name = "UserDtoList", params(UserDto)))]
#[graphql(concrete(name = "UserRepoInfoDtoList", params(UserRepoInfoDto)))]
pub struct DtoList<T>
where
    T: utoipa::ToSchema + async_graphql::OutputType,
{
    pub dtos: Vec<T>,
    pub count: u64,
    pub last_taken_entity_number: Option<u64>,
}

impl<T> DtoList<T>
where
    T: utoipa::ToSchema + async_graphql::OutputType,
{
    pub fn new(dtos: Vec<T>, count: u64, take: Option<u64>, offset: Option<u64>) -> Self {
        let last_taken_entity_number = match (take, offset) {
            (None, None) => Some(count),
            (None, Some(offset)) => {
                if offset >= count {
                    None
                } else {
                    Some(count)
                }
            }

            (Some(take), Some(offset)) => {
                let last_taken_number = take + offset;
                if offset > count {
                    None
                } else if last_taken_number > count || take == 0 {
                    Some(count)
                } else {
                    Some(last_taken_number)
                }
            }

            (Some(take), None) => {
                if take > count || take == 0 {
                    Some(count)
                } else {
                    Some(take)
                }
            }
        };
        Self {
            dtos,
            count,
            last_taken_entity_number,
        }
    }
}

#[derive(
    Serialize, Deserialize, PartialEq, Debug, async_graphql::SimpleObject, utoipa::ToSchema,
)]
#[graphql(concrete(name = "UserToRepoDto", params(UserDto, RepoDto)))]
pub struct OneToOneDto<L, R>
where
    L: async_graphql::OutputType + utoipa::ToSchema,
    R: async_graphql::OutputType + utoipa::ToSchema,
{
    pub left: L,
    pub right: R,
}

impl<L, R> OneToOneDto<L, R>
where
    L: async_graphql::OutputType + utoipa::ToSchema,
    R: async_graphql::OutputType + utoipa::ToSchema,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

#[derive(
    Serialize, Deserialize, PartialEq, Debug, async_graphql::SimpleObject, utoipa::ToSchema,
)]
#[graphql(concrete(name = "UserToReposDto", params(UserDto, RepoDto)))]
pub struct OneToManyDto<O, M>
where
    O: async_graphql::OutputType + utoipa::ToSchema,
    M: async_graphql::OutputType + utoipa::ToSchema,
    DtoList<M>: async_graphql::OutputType + utoipa::ToSchema,
{
    pub one: O,
    pub many: DtoList<M>,
}

impl<O, M> OneToManyDto<O, M>
where
    O: async_graphql::OutputType + utoipa::ToSchema,
    M: async_graphql::OutputType + utoipa::ToSchema,
    DtoList<M>: async_graphql::OutputType + utoipa::ToSchema,
{
    pub fn new(one: O, many: DtoList<M>) -> Self {
        Self { one, many }
    }
}
