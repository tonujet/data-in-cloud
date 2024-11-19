use mongodb::bson::doc;
use collection::user::User;
use crate::dao::error::RepoResult;

use crate::dao::RepositoryTrait;
use crate::dao::user_repo::UserRepository;
use crate::dto::user_dto::UserDto;
use crate::utils::user::user_test_helper;

#[tokio::test]
async fn list_all_not_deleted_users_success() {
    let repo = user_test_helper::get_mock_repo();
    let create_dtos = user_test_helper::get_create_dtos();
    let mut ids = Vec::new();
    let mut counter = create_dtos.len();
    let take = 0;
    let skip = 0;

    for dto in create_dtos {
        let created_dto = repo.create(dto).await;
        assert!(created_dto.is_ok());
        ids.push(created_dto.unwrap().id.unwrap())
    }

    for (i, id) in ids.into_iter().enumerate() {
        if i % 2 == 0 {
            let deleted = repo.delete(&id).await;
            assert!(deleted.is_ok());
            counter -= 1;
        }
    }

    let created_dtos = repo.list(Some(take), Some(skip)).await;
    assert!(created_dtos.is_ok());
    let created_dtos = created_dtos.unwrap();
    assert_eq!(created_dtos.count, counter as u64)
}

#[tokio::test]
async fn get_deleted_user_failure() {
    let repo = user_test_helper::get_mock_repo();
    let created_dto = run_create_dto1(&repo).await;

    let deleted_dto = repo.delete(&created_dto.id.unwrap()).await;
    assert!(deleted_dto.is_ok());

    let doc = repo
        .get_user(doc! {"_id": deleted_dto.unwrap().id.unwrap()})
        .await;
    assert!(doc.is_err())
}

#[tokio::test]
async fn get_user_success() {
    let repo = user_test_helper::get_mock_repo();
    let created_dto = run_create_dto1(&repo).await;
    
    let doc = repo
        .get_user(doc! {"_id": created_dto.id.unwrap()})
        .await;
    assert!(doc.is_ok())
}

#[tokio::test]
async fn analyze_responses_to_uniqueness_success() {
    let repo = user_test_helper::get_mock_repo();
    let created_dto = run_create_dto1(&repo).await;
    let key_field = "email";

    let email_res = repo
        .get_user(doc! {key_field: created_dto.email})
        .await;

    let reses = vec![(email_res, key_field)];
    let analized = repo.analyze_reses_to_uniqueness(reses);
    assert!(analized.is_err())
}

#[tokio::test]
async fn analyze_responses_to_uniqueness_failure() {
    let repo = user_test_helper::get_mock_repo();
    let reses: Vec<(RepoResult<User>, &'static str)> = vec![];
    
    let analized = repo.analyze_reses_to_uniqueness(reses);
    
    assert!(analized.is_ok())
}


#[tokio::test]
async fn validate_update_uniqueness_success(){
    let repo = user_test_helper::get_mock_repo();
    let update_dto = user_test_helper::get_update_dto();
    let user = user_test_helper::get_created1();
    
    let validated = repo.validate_update_uniqueness(&user, &update_dto).await;
    
    assert!(validated.is_ok())
}



#[tokio::test]
async fn validate_update_uniqueness_failure(){
    let repo = user_test_helper::get_mock_repo();
    let _ = run_create_dto1(&repo).await;
    let created_dto2 = run_create_dto2(&repo).await;
    
    let mut update_dto = user_test_helper::get_update_dto();
    update_dto.username = created_dto2.username.clone();
    let user = user_test_helper::get_created1();
    
    let validated = repo.validate_update_uniqueness(&user, &update_dto).await;

    assert!(validated.is_err())
}


async fn run_create_dto1(repo: &UserRepository) -> UserDto {
    let create_dto = user_test_helper::get_create_dto1();
    let created_dto = repo.create(create_dto).await;
    assert!(created_dto.is_ok());
    return created_dto.unwrap()
}

async fn run_create_dto2(repo: &UserRepository) -> UserDto {
    let create_dto = user_test_helper::get_create_dto2();
    let created_dto = repo.create(create_dto).await;
    assert!(created_dto.is_ok());
    return created_dto.unwrap()
}