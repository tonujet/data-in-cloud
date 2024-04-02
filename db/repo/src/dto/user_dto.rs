use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use collection::user::User;

use crate::utils::dto::serialize_object_id;

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 200, message = "Must be between 3 and 30 characters"))]
    #[validate(email(message = "Must be email with the common pattern"))]
    pub email: String,

    #[validate(length(min = 3, max = 200, message = "Must be between 3 and 30 characters"))]
    pub username: String,

    #[validate(length(min = 3, max = 200, message = "Must be between 3 and 30 characters"))]
    pub password: String,

    #[validate(range(min = 0, max = 255))]
    pub age: u8,

    pub is_public: bool,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserDto {
    #[validate(length(min = 3, max = 200, message = "Must be between 3 and 30 characters"))]
    pub username: String,

    #[validate(range(min = 0, max = 255))]
    pub age: u8,
    pub is_public: bool,
}

impl From<CreateUserDto> for User {
    fn from(
        CreateUserDto {
            email,
            username,
            password,
            age,
            is_public,
        }: CreateUserDto,
    ) -> Self {
        let now = Local::now();
        User::new(email, username, password, age, is_public, now, now)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UserDto {
    #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub email: String,
    pub username: String,
    pub age: u8,
    pub is_public: bool,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

impl From<User> for UserDto {
    fn from(
        User {
            id,
            email,
            username,
            password: _password,
            deleted: _deleted,
            age,
            is_public,
            created,
            updated,
        }: User,
    ) -> Self {
        UserDto {
            id,
            email,
            username,
            age,
            is_public,
            created,
            updated,
        }
    }
}


impl PartialEq for UserDto {
    fn eq(&self, other: &Self) -> bool {
        let Self {
            id: _id, email, username, age, is_public, ..
        } = other;
        self.email == *email && self.username == *username && self.age == *age && self.is_public == *is_public
    }
}

