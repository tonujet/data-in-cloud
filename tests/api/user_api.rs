use axum::http::StatusCode;
use axum_test::TestServer;
use serial_test::serial;

use repo::dto::DtoList;
use repo::dto::user_dto::UserDto;
use repo::utils::user::user_test_helper;

use crate::common::Setup;

#[tokio::test]
#[serial]
async fn create_user_success() {
    let setup = Setup::new().await;
    create_user1(&setup.client).await;
}

#[tokio::test]
#[serial]
async fn create_user_with_taken_email_and_username_failure() {
    let setup = Setup::new().await;
    let create_dto = user_test_helper::get_create_dto1();
    let expected_code = StatusCode::CONFLICT;

    create_user1(&setup.client).await;
    let res = setup.client.post("/apiV1/users").json(&create_dto).await;

    assert_eq!(res.status_code(), expected_code);
}

#[tokio::test]
#[serial]
async fn update_user_success() {
    let setup = Setup::new().await;
    let update_dto = user_test_helper::get_update_dto();
    let expected_code = StatusCode::OK;
    let expected_user_dto = user_test_helper::get_updated_dto();

    let user_dto = create_user1(&setup.client).await;
    let res = setup
        .client
        .put(&format!("/apiV1/users/{}", user_dto.id.unwrap()))
        .json(&update_dto)
        .await;
    let user_dto: UserDto = res.json();

    assert_eq!(res.status_code(), expected_code);
    assert_eq!(user_dto, expected_user_dto)
}

#[tokio::test]
#[serial]
async fn update_user_with_taken_username_failure() {
    let setup = Setup::new().await;
    let expected_code = StatusCode::CONFLICT;

    let user1_dto = create_user1(&setup.client).await;
    let user2_dto = create_user2(&setup.client).await;
    let mut update_dto = user_test_helper::get_update_dto();
    update_dto.username = user1_dto.username;
    let res = setup
        .client
        .put(&format!("/apiV1/users/{}", user2_dto.id.unwrap()))
        .json(&update_dto)
        .await;

    assert_eq!(res.status_code(), expected_code)
}

#[tokio::test]
#[serial]
async fn delete_user_success() {
    let setup = Setup::new().await;
    let expected_delete_code = StatusCode::OK;
    let expected_get_code = StatusCode::CONFLICT;
    let user_dto = create_user1(&setup.client).await;

    let res = setup
        .client
        .delete(&format!("/apiV1/users/{}", user_dto.id.unwrap()))
        .await;
    let deleted_dto: UserDto = res.json();
    
    assert_eq!(res.status_code(), expected_delete_code);
    assert_eq!(deleted_dto, user_dto);

    let res = setup
        .client
        .get(&format!("/apiV1/users/{}", user_dto.id.unwrap()))
        .await;
    assert_eq!(res.status_code(), expected_get_code);
}

#[tokio::test]
#[serial]
async fn delete_nonexistent_user_failure() {
    let setup = Setup::new().await;
    let user_dto = user_test_helper::get_created1();
    let expected_delete_code = StatusCode::CONFLICT;

    let res = setup
        .client
        .delete(&format!("/apiV1/users/{}", user_dto.id.unwrap()))
        .await;
    
    assert_eq!(res.status_code(), expected_delete_code);
}

#[tokio::test]
#[serial]
async fn get_user_success() {
    let setup = Setup::new().await;
    let user_dto = create_user1(&setup.client).await;
    let expected_code = StatusCode::OK;

    let res = setup
        .client
        .get(&format!("/apiV1/users/{}", user_dto.id.unwrap()))
        .await;
    let got_dto: UserDto = res.json();

    assert_eq!(res.status_code(), expected_code);
    assert_eq!(got_dto, user_dto);
}

#[tokio::test]
#[serial]
async fn get_notexistent_user_failure() {
    let setup = Setup::new().await;
    let user_dto = user_test_helper::get_created_dto1();
    let expected_code = StatusCode::CONFLICT;

    let res = setup
        .client
        .get(&format!("/apiV1/users/{}", user_dto.id.unwrap()))
        .await;

    assert_eq!(res.status_code(), expected_code)
}

#[tokio::test]
#[serial]
async fn list_all_users_success() {
    let setup = Setup::new().await;
    let expected_code = StatusCode::OK;
    let created_dtos = create_users(&setup.client).await;
    let expected_count = created_dtos.len() as u64;

    
    let res = setup.client.get("/apiV1/users").await;
    let dtos: DtoList<UserDto> = res.json();

    assert_eq!(res.status_code(), expected_code);
    assert_eq!(dtos.count, expected_count);
}

#[tokio::test]
#[serial]
async fn list_users_using_take_and_skip_success() {
    let setup = Setup::new().await;
    let expected_code = StatusCode::OK;
    let created_dtos = create_users(&setup.client).await;
    let expected_count = created_dtos.len() as u64;
    let expected_len = created_dtos[1..4].len();

    let res = setup
        .client
        .get("/apiV1/users")
        .add_query_param("take", 3)
        .add_query_param("offset", 1)
        .await;
    let dtos: DtoList<UserDto> = res.json();
    
    assert_eq!(res.status_code(), expected_code);
    assert_eq!(dtos.dtos.len(), expected_len);
    assert_eq!(dtos.count, expected_count);
}

async fn create_user1(client: &TestServer) -> UserDto {
    let create_dto = user_test_helper::get_create_dto1();
    let expected_status_code = StatusCode::OK;
    let expected_user_dto = user_test_helper::get_created_dto1();

    let res = client.post("/apiV1/users").json(&create_dto).await;
    let user_dto: UserDto = res.json();

    assert_eq!(res.status_code(), expected_status_code);
    assert_eq!(user_dto, expected_user_dto);

    user_dto
}

async fn create_user2(client: &TestServer) -> UserDto {
    let create_dto = user_test_helper::get_create_dto2();
    let expected_status_code = StatusCode::OK;
    let expected_created_dto = user_test_helper::get_created_dto2();

    let res = client.post("/apiV1/users").json(&create_dto).await;
    let created_dto: UserDto = res.json();

    assert_eq!(res.status_code(), expected_status_code);
    assert_eq!(created_dto, expected_created_dto);
    created_dto
}

async fn create_users(client: &TestServer) -> Vec<UserDto> {
    let create_dtos = user_test_helper::get_create_dtos();
    let expected_status_code = StatusCode::OK;
    let mut created_dtos = Vec::new();

    for dto in create_dtos {
        let res = client.post("/apiV1/users").json(&dto).await;
        assert_eq!(res.status_code(), expected_status_code);

        let user_dto: UserDto = res.json();
        created_dtos.push(user_dto);
    }

    created_dtos
}
