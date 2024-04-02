use serde::{Deserialize, Serialize};
use repo::dto::DtoList;
use repo::dto::repository_dto::RepoDto;
use repo::dto::user_dto::UserDto;

#[derive(Serialize, Deserialize)]
pub struct UserSingleRepo {
    pub user: UserDto,
    pub repo: RepoDto,
}

impl UserSingleRepo {
    pub fn new(user: UserDto, repo: RepoDto) -> Self {
        Self { user, repo }
    }
}



#[derive(Serialize, Deserialize)]
pub struct UserMultipleRepo {
    pub user: UserDto,
    pub repos: DtoList<RepoDto>,
}

impl UserMultipleRepo {
    pub fn new(user: UserDto, repos: DtoList<RepoDto>) -> Self {
        Self { user, repos }
    }
}