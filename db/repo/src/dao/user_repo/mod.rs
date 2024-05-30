use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Local, Utc};
use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;

use collection::MongoCollection;
use collection::user::User;

use crate::dto::{
    DtoList,
    user_dto::UpdateUserDto
};

use super::{RepositoryTrait, UserRepositoryTrait};
use super::{CreateUserDto, UserDto};
use super::error::{
    Entity,
    RepoError::{DeletedWithObjectId, Internal, Uniqueness}, RepoResult
};

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct UserRepository {
    pub collection: Arc<dyn MongoCollection<User>>,
}

impl UserRepositoryTrait for UserRepository {}

impl UserRepository {
    async fn get_user(&self, document: Document) -> RepoResult<User> {
        let user = self
            .collection
            .find_one(Some(document), None)
            .await?
            .ok_or(Internal("Can not find user"))?;
        if user.deleted {
            Err(DeletedWithObjectId(user.id.unwrap(), Entity::User))?
        }
        Ok(user)
    }

    async fn validate_create_uniqueness(
        &self,
        CreateUserDto {
            email, username, ..
        }: &CreateUserDto,
    ) -> RepoResult<()> {
        let email_res = self.get_user(doc! {"email": email}).await;
        let username_res = self.get_user(doc! {"username": username}).await;

        let reses = vec![(email_res, "email"), (username_res, "username")];
        Ok(self.analyze_reses_to_uniqueness(reses)?)
    }

    async fn validate_update_uniqueness(
        &self,
        user: &User,
        user_dto: &UpdateUserDto,
    ) -> RepoResult<()> {
        let mut taken_fields = vec![];

        if user.username != user_dto.username {
            let username_res = self.get_user(doc! {"username": &user_dto.username}).await;
            taken_fields.push((username_res, "username"));
        }
        Ok(self.analyze_reses_to_uniqueness(taken_fields)?)
    }

    fn analyze_reses_to_uniqueness(
        &self,
        reses: Vec<(RepoResult<User>, &'static str)>,
    ) -> RepoResult<()> {
        let taken_fields: Vec<&'static str> = reses
            .iter()
            .filter(|tup| tup.0.is_ok())
            .map(|tup| tup.1)
            .collect();
        if taken_fields.len() != 0 {
            Err(Uniqueness(taken_fields, Entity::User))?
        }
        Ok(())
    }
}

#[async_trait]
impl RepositoryTrait<CreateUserDto, UpdateUserDto, UserDto, ObjectId> for UserRepository {
    async fn create(&self, dto: CreateUserDto) -> RepoResult<UserDto> {
        self.validate_create_uniqueness(&dto).await?;
        let user = User::from(dto);
        let id = self.collection.insert_one(user, None).await?;
        let user = self.get_user(doc! {"_id": id}).await?;
        Ok(user.into())
    }

    async fn update(&self, id: &ObjectId, dto: UpdateUserDto) -> RepoResult<UserDto> {
        let user = self.get_user(doc! {"_id": id}).await?;
        self.validate_update_uniqueness(&user, &dto).await?;
        let UpdateUserDto {
            username,
            age,
            is_public,
        } = dto;

        let filter = doc! {"_id": id};
        let update = doc! {"$set": doc! {
            "username": username,
            "age": age as u32,
            "is_public": is_public,
            "updated": Utc::now().to_rfc3339()
        }};

        self.collection
            .update_one(filter, update.into(), None)
            .await?;
        let user = self.get_user(doc! {"_id": id}).await?;
        Ok(user.into())
    }

    async fn delete(&self, id: &ObjectId) -> RepoResult<UserDto> {
        let filter = doc! {"_id": id};
        let user = self.get_user(filter.clone()).await?;
        let update = doc! {"$set": doc! {
            "deleted": true,
        }};
        self.collection
            .update_one(filter, update.into(), None)
            .await?;
        Ok(user.into())
    }

    async fn get(&self, id: &ObjectId) -> RepoResult<UserDto> {
        let user = self.get_user(doc! {"_id": id}).await?;
        Ok(user.into())
    }

    async fn list(&self, take: Option<u64>, offset: Option<u64>) -> RepoResult<DtoList<UserDto>> {
        let mut pipeline = vec![
            doc! {"$match": doc!{"deleted": false}},
            doc! {"$skip" : offset.unwrap_or(0) as u32},
        ];

        if let Some(take) = take.filter(|&take| take != 0) {
            pipeline.push(doc! {"$limit" : take as u32})
        }

        let dtos = self
            .collection
            .aggregate_and_collect(pipeline, None)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();

        let count = self
            .collection
            .count_documents(Some(doc! {"deleted": false}), None)
            .await?;
        Ok(DtoList::new(dtos, count, take, offset))
    }
}
