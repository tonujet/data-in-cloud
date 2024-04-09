use std::sync::Arc;
use mongodb::bson::oid::ObjectId;

use object_store::memory::InMemory;
use uuid::Uuid;

use crate::dao::user_repository_repo::UserRepositoryRepo;
use crate::dto::repository_dto::CreateUpdateRepoDto;
use crate::dto::user_dto::CreateUserDto;
use crate::utils::repository::repository_test_helper;
use crate::utils::user::user_test_helper;

pub fn get_mock_repo() -> UserRepositoryRepo {
    let store = Arc::new(InMemory::new());
    let repo = UserRepositoryRepo::new(store);
    repo
}


pub fn get_mock_repo_with_starter() -> (UserRepositoryRepo, ObjectId, Uuid) {
    let repo = get_mock_repo();
    let user_id = ObjectId::new();
    let repo_id = Uuid::new_v4();
    (repo, user_id, repo_id)
}


pub fn get_create_dtos() -> (CreateUserDto, CreateUpdateRepoDto) {
    let repo_create_dto = repository_test_helper::get_create_dto();
    let user_create_dto = user_test_helper::get_create_dto1();
    (user_create_dto, repo_create_dto)
}








