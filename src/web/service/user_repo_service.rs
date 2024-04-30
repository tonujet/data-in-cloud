use std::sync::Arc;

use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;

use collection::user_repo_info::UserRepoInfoOperation;
use repo::dao::UserRepoRepositoryTrait;
use repo::dto::DtoList;
use repo::dto::user_repo_info_dto::CreateUserRepoInfoDto;

use crate::message_broker;
use crate::message_broker::Publisher;
use crate::web::dto::user_repo_dto::{UserMultipleRepo, UserSingleRepo};
use crate::web::error::ApiResult;
use crate::web::service::{BlobConnServiceTrait, RepoServiceTrait, UserRepoServiceTrait, UserServiceTrait};


#[derive(Clone)]
pub struct UserRepoService {
    repo: Arc<dyn UserRepoRepositoryTrait>,
    user_service: Arc<dyn UserServiceTrait>,
    repo_service: Arc<dyn RepoServiceTrait>,
    info_publisher: Arc<dyn Publisher<CreateUserRepoInfoDto>>,
}

impl UserRepoService {
    pub fn new(
        repo: Arc<dyn UserRepoRepositoryTrait>,
        user_service: Arc<dyn UserServiceTrait>,
        repo_service: Arc<dyn RepoServiceTrait>,
        info_publisher: Arc<dyn Publisher<CreateUserRepoInfoDto>>,
    ) -> Self {
        Self {
            repo,
            user_service,
            repo_service,
            info_publisher,
        }
    }
}

impl UserRepoServiceTrait for UserRepoService {}

#[async_trait]
impl BlobConnServiceTrait<ObjectId, Uuid, UserSingleRepo, UserMultipleRepo> for UserRepoService {
    async fn add_pair(&self, key_id: &ObjectId, val_id: &Uuid) -> ApiResult<UserSingleRepo> {
        let user = self.user_service.get(key_id).await?;
        let repo = self.repo_service.get(val_id).await?;
        self.repo.add_pair(key_id, val_id).await?;
        self.info_publisher
            .publish(CreateUserRepoInfoDto {
                user_id: user.id.unwrap(),
                repo_id: repo.id,
                operation: UserRepoInfoOperation::CreateLink,
            })
            .await?;
        Ok(UserSingleRepo::new(user, repo))
    }

    async fn list_pairs(
        &self,
        key_id: &ObjectId,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> ApiResult<UserMultipleRepo> {
        let user = self.user_service.get(key_id).await?;
        let repo_ids = self.repo.list_pairs(key_id).await?;
        let len = repo_ids.len();
        let offset_num = offset.unwrap_or(0) as usize;
        let take_num = take
            .filter(|&n| n != 0)
            .map(|n| n as usize)
            .unwrap_or(repo_ids.len());

        let repo_ids: Vec<_> = repo_ids
            .into_iter()
            .skip(offset_num)
            .take(take_num)
            .collect();

        let mut repos = vec![];
        for id in &repo_ids {
            let repo = self.repo_service.get(id).await?;
            repos.push(repo);
        }
        Ok(UserMultipleRepo::new(
            user,
            DtoList::new(repos, len as u64, take, offset),
        ))
    }

    async fn delete_pair(&self, key_id: &ObjectId, val_id: &Uuid) -> ApiResult<UserSingleRepo> {
        let user = self.user_service.get(key_id).await?;
        let repo = self.repo_service.get(val_id).await?;
        self.repo.delete_pair(key_id, val_id).await?;
        self.info_publisher
            .publish(CreateUserRepoInfoDto {
                user_id: user.id.unwrap(),
                repo_id: repo.id,
                operation: UserRepoInfoOperation::DeleteLink,
            })
            .await?;
        Ok(UserSingleRepo::new(user, repo))
    }
}
