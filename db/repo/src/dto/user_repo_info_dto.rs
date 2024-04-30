use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use collection::user_repo_info::{UserRepoInfo, UserRepoInfoOperation};

use crate::utils::dto::{serialize_object_id, serialize_option_object_id};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRepoInfoDto {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_object_id"
    )]
    pub id: Option<ObjectId>,

    #[serde(serialize_with = "serialize_object_id")]
    pub user_id: ObjectId,
    pub repo_id: Uuid,
    pub operation: UserRepoInfoOperation,
    pub executed_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRepoInfoDto {
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

impl Into<Vec<u8>> for CreateUserRepoInfoDto {
    fn into(self) -> Vec<u8> {
        let string = serde_json::to_string(&self).unwrap();
        string.into_bytes()
    }
}
