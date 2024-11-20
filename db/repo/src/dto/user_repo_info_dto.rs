use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use collection::user_repo_info::{UserRepoInfo, UserRepoInfoOperation};

use crate::utils::dto::{serialize_object_id, serialize_option_object_id, object_id_schema};

#[derive(Serialize, Deserialize, Debug)]
#[derive(async_graphql::SimpleObject)]
#[derive(utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UserRepoInfoDto {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id"
    )]
    #[schema(schema_with = object_id_schema)]
    pub id: Option<ObjectId>,

    #[serde(serialize_with = "serialize_object_id")]
    #[schema(schema_with = object_id_schema)]
    pub user_id: ObjectId,
    pub repo_id: Uuid,
    pub operation: UserRepoInfoOperation,
    pub executed_at: DateTime<Utc>,
}


impl PartialEq for UserRepoInfoDto {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id && self.repo_id == other.repo_id &&  self.operation == other.operation
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(utoipa::ToSchema)]
pub struct CreateUserRepoInfoDto {
    #[schema(schema_with = object_id_schema)]
    pub user_id: ObjectId,
    pub repo_id: Uuid,
    pub operation: UserRepoInfoOperation,
}

impl From<CreateUserRepoInfoDto> for UserRepoInfo {
    fn from(
        CreateUserRepoInfoDto {
            user_id,
            repo_id,
            operation,
        }: CreateUserRepoInfoDto,
    ) -> Self {
        Self::new(user_id, repo_id, operation)
    }
}

impl From<UserRepoInfo> for UserRepoInfoDto {
    fn from(
        UserRepoInfo {
            id,
            user_id,
            repo_id,
            operation,
            executed_at,
        }: UserRepoInfo,
    ) -> Self {
        UserRepoInfoDto {
            id,
            user_id,
            repo_id,
            operation,
            executed_at,
        }
    }
}

impl From<Vec<u8>> for CreateUserRepoInfoDto {
    fn from(value: Vec<u8>) -> Self {
        let str = String::from_utf8(value).unwrap();
        let dto: CreateUserRepoInfoDto = serde_json::from_str(&str).unwrap();
        dto
    }
}


impl From<CreateUserRepoInfoDto> for Vec<u8> {
    fn from(val: CreateUserRepoInfoDto) -> Self {
        let string = serde_json::to_string(&val).unwrap();
        string.into_bytes()
    }
}