use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect,
};
use uuid::Uuid;

use entity::{repository, repository::Entity as Repository};
use entity::repository::Model;

use super::{CreateUpdateRepoDto, ResponseRepoDto};
use super::error::{Entity, RepoError, RepoResult};
use super::ListResponse;
use super::RepositoryRepoTrait;
use super::utils::repository::repository_mapper::{map_entities_to_dtos, map_entity_to_dto};

#[cfg(test)]
mod tests;

pub struct RepositoryRepo {
    conn: DbConn,
}

impl RepositoryRepo {
    pub fn new(conn: DbConn) -> Self {
        Self { conn }
    }

    async fn get_repo_model(&self, id: Uuid) -> RepoResult<Model> {
        Repository::find_by_id(id)
            .one(&self.conn)
            .await?
            .ok_or(RepoError::NotFoundWithUuid(id, Entity::Repository))
    }

    async fn get_active_repo_model(&self, id: Uuid) -> RepoResult<repository::ActiveModel> {
        let repo = self.get_repo_model(id).await?;
        self.is_repo_deleted(&repo)?;
        let repo: repository::ActiveModel = repo.into();
        Ok(repo)
    }

    fn is_repo_deleted(&self, repo: &Model) -> RepoResult<()> {
        if repo.deleted {
            return Err(RepoError::DeletedWithUuid(repo.id, Entity::Repository));
        }
        Ok(())
    }
}

#[async_trait]
impl RepositoryRepoTrait for RepositoryRepo {
    async fn create(&self, repo_dto: CreateUpdateRepoDto) -> RepoResult<ResponseRepoDto> {
        let repo = repository::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(repo_dto.title),
            description: Set(repo_dto.description),
            r#type: Set(repo_dto.repo_type),
            location: Set("unknown".to_string()),
            ..Default::default()
        };

        let repo = repo.insert(&self.conn).await?;

        Ok(map_entity_to_dto(repo))
    }

    async fn update(&self, id: Uuid, repo_dto: CreateUpdateRepoDto) -> RepoResult<ResponseRepoDto> {
        let mut repo = self.get_active_repo_model(id).await?;

        let CreateUpdateRepoDto {
            title,
            description,
            repo_type,
        } = repo_dto;

        repo.title = Set(title);
        repo.description = Set(description);
        repo.r#type = Set(repo_type);

        let repo = repo.update(&self.conn).await?;
        Ok(map_entity_to_dto(repo))
    }

    async fn delete(&self, id: Uuid) -> RepoResult<ResponseRepoDto> {
        let mut repo = self.get_active_repo_model(id).await?;
        repo.deleted = Set(true);
        let repo = repo.update(&self.conn).await?;
        Ok(map_entity_to_dto(repo))
    }

    async fn get(&self, id: Uuid) -> RepoResult<ResponseRepoDto> {
        let repo = self.get_repo_model(id).await?;
        self.is_repo_deleted(&repo)?;
        Ok(map_entity_to_dto(repo))
    }

    async fn list(
        &self,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> RepoResult<ListResponse<ResponseRepoDto>> {
        let count = Repository::find()
            .filter(repository::Column::Deleted.eq(false))
            .count(&self.conn)
            .await?;

        let repos = Repository::find()
            .filter(repository::Column::Deleted.eq(false))
            .limit(take)
            .offset(offset)
            .all(&self.conn)
            .await?;
        let dtos = map_entities_to_dtos(repos);
        Ok(ListResponse::new(dtos, count, take, offset))
    }
}
