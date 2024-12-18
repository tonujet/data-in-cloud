use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};

use collection::user::User;
use collection::MongoCollection;

use dto::{user_dto::UpdateUserDto, DtoList};

use super::error::{
    Entity,
    RepoError::{DeletedWithObjectId, Internal, Uniqueness},
    RepoResult,
};
use super::{CreateUserDto, UserDto};
use super::{RepositoryTrait, UserRepositoryTrait};

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct UserRepository {
    collection: Arc<dyn MongoCollection<User>>,
}

impl UserRepository {
    pub fn new(collection: Arc<dyn MongoCollection<User>>) -> Self {
        Self { collection }
    }
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
        self.analyze_reses_to_uniqueness(reses)
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
        self.analyze_reses_to_uniqueness(taken_fields)
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
        if !taken_fields.is_empty() {
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
        let pipeline = vec![doc! {"$match": doc!{"deleted": false}}];

        let dtos = self
            .collection
            .paginate_pipeline_and_collect(pipeline, take, offset, None)
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
