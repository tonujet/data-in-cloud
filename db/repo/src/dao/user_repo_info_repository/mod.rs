use std::sync::Arc;

use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

use collection::user_repo_info::UserRepoInfo;
use collection::MongoCollection;

use crate::dao::error::RepoError::Internal;
use crate::dao::error::RepoResult;
use crate::dao::{PersistentRepositoryTrait, UserRepoInfoRepositoryTrait};
use crate::dto::user_repo_info_dto::{CreateUserRepoInfoDto, UserRepoInfoDto};
use crate::dto::DtoList;

#[cfg(test)]
mod tests;

pub struct UserRepoInfoRepository {
    pub collection: Arc<dyn MongoCollection<UserRepoInfo>>,
}

#[async_trait]
impl PersistentRepositoryTrait<CreateUserRepoInfoDto, UserRepoInfoDto, ObjectId>
    for UserRepoInfoRepository
{
    async fn create(&self, dto: CreateUserRepoInfoDto) -> RepoResult<UserRepoInfoDto> {
        let info = UserRepoInfo::from(dto);
        let id = self.collection.insert_one(info, None).await?;
        Ok(self.get(&id).await?)
    }

    async fn get(&self, id: &ObjectId) -> RepoResult<UserRepoInfoDto> {
        let info = self
            .collection
            .find_one(Some(doc! {"_id": id}), None)
            .await?
            .ok_or(Internal("Can not find user"))?;
        Ok(info.into())
    }

    async fn list(
        &self,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> RepoResult<DtoList<UserRepoInfoDto>> {
        let mut pipeline = vec![doc! {"$skip" : offset.unwrap_or(0) as u32}];

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

        let count = self.collection.count_documents(None, None).await?;
        Ok(DtoList::new(dtos, count, take, offset))
    }
}

#[async_trait]
impl UserRepoInfoRepositoryTrait for UserRepoInfoRepository {
    async fn list_by_user_id(
        &self,
        user_id: ObjectId,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> RepoResult<DtoList<UserRepoInfoDto>> {
        let pipeline = vec![doc! {"$match": doc!{"user_id": user_id}}];

        let dtos = self
            .collection
            .paginate_pipeline_and_collect(pipeline, take, offset, None)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();

        let count = self
            .collection
            .count_documents(None, None)
            .await?;
        
        Ok(DtoList::new(dtos, count, take, offset))
    }
}
