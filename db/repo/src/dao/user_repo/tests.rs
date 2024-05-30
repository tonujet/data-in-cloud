use mongodb::bson::doc;

use collection::user::User;

use crate::dao::error::RepoResult;
use crate::dao::RepoTrait;
use crate::dao::user_repo::UserRepo;
use crate::dto::user_dto::UserDto;
use crate::utils::user::user_test_helper;


#[tokio::test]
async fn list_all_not_deleted_users_success() {
    let repo = user_test_helper::get_mock_repo();
    let create_dtos = user_test_helper::get_create_dtos();
    let mut created_dtos = vec![];
    let take = 0;
    let skip = 0;
    let mut expected_dtos = vec![];

    for dto in create_dtos {
        let created_dto = repo.create(dto).await;
        created_dtos.push(created_dto.unwrap())
    }

    for (i, dto) in created_dtos.into_iter().enumerate() {
        if i % 2 == 0 {
            let _ = repo.delete(&dto.id.unwrap()).await;
        } else {
            expected_dtos.push(dto)
        }
    }

    let left_dtos = repo.list(Some(take), Some(skip)).await;
    assert!(left_dtos.is_ok());
    assert_eq!(left_dtos.unwrap().dtos, expected_dtos);
}

#[tokio::test]
async fn get_deleted_user_failure() {
    let repo = user_test_helper::get_mock_repo();
    let created_dto = run_create_dto1(&repo).await;
    
    
    let deleted_dto = repo.delete(&created_dto.id.unwrap()).await;
    let deleted_dto = deleted_dto.unwrap();
    assert_eq!(deleted_dto, created_dto);
    
    let doc = repo
        .get_user(doc! {"_id": deleted_dto.id.unwrap()})
        .await;
    
    assert!(doc.is_err())
}

#[tokio::test]
async fn get_user_success() {
    let repo = user_test_helper::get_mock_repo();
    let UserDto {
        id: dto_id,
        email: dto_email,
        username: dto_username,
        age: dto_age,
        is_public: dto_public,
        ..
    } = run_create_dto1(&repo).await;

    let doc = repo.get_user(doc! {"_id": dto_id.unwrap()}).await;

    assert!(doc.is_ok());
    let User {
        id,
        email,
        username,
        password: _password,
        age,
        is_public,
        ..
    } = doc.unwrap();
    assert!(
        id == dto_id
            && email == dto_email
            && username == dto_username
            && age == dto_age
            && is_public == dto_public
    );
}

#[tokio::test]
async fn analyze_responses_to_uniqueness_success() {
    let repo = user_test_helper::get_mock_repo();
    let created_dto = run_create_dto1(&repo).await;
    let key_field = "email";

    let email_res = repo.get_user(doc! {key_field: created_dto.email}).await;
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
async fn validate_update_uniqueness_success() {
    let repo = user_test_helper::get_mock_repo();
    let update_dto = user_test_helper::get_update_dto();
    let user = user_test_helper::get_created1();

    let validated = repo.validate_update_uniqueness(&user, &update_dto).await;

    assert!(validated.is_ok())
}

#[tokio::test]
async fn validate_update_uniqueness_failure() {
    let repo = user_test_helper::get_mock_repo();
    let _ = run_create_dto1(&repo).await;
    let created_dto2 = run_create_dto2(&repo).await;

    let mut update_dto = user_test_helper::get_update_dto();
    update_dto.username = created_dto2.username.clone();
    let user = user_test_helper::get_created1();

    let validated = repo.validate_update_uniqueness(&user, &update_dto).await;

    assert!(validated.is_err())
}

async fn run_create_dto1(repo: &UserRepo) -> UserDto {
    let create_dto = user_test_helper::get_create_dto1();
    let created_dto = repo.create(create_dto).await;
    assert!(created_dto.is_ok());
    created_dto.unwrap()
}

async fn run_create_dto2(repo: &UserRepo) -> UserDto {
    let create_dto = user_test_helper::get_create_dto2();
    let created_dto = repo.create(create_dto).await;
    assert!(created_dto.is_ok());
    created_dto.unwrap()
}
