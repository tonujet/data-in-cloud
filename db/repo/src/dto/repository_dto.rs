use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use entity::{repository, RepositoryType};


#[derive(Deserialize, Debug, Validate, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateUpdateRepoDto {
    #[validate(length(min = 3, max = 30, message = "Must be between 3 and 30 characters"))]
    pub title: String,

    #[validate(length(min = 3, max = 1000, message = "Must be between 3 and 1000 characters"))]
    pub description: Option<String>,
    pub repo_type: RepositoryType,
}

#[derive(Serialize, Deserialize, Debug, Eq, Clone)]
pub struct RepoDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub repo_type: RepositoryType,
    pub stars: u64,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

impl PartialEq for RepoDto {
    fn eq(&self, o: &Self) -> bool {
        let Self {
            id: _id,
            title,
            description,
            repo_type,
            stars,
            ..
        } = &self;
        o.title == *title
            && o.description == *description
            && o.repo_type == *repo_type
            && o.stars == *stars
    }
}

impl From<repository::Model> for RepoDto {
    fn from(repo: repository::Model) -> Self {
        let repository::Model {
            id,
            title,
            description,
            deleted: _deleted,
            r#type: repo_type,
            stars,
            location: _location,
            created,
            updated,
        } = repo;

        RepoDto {
            id,
            title,
            description,
            repo_type,
            stars: stars as u64,
            created,
            updated,
        }
    }
}
