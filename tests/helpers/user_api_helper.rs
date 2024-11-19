use axum::http::StatusCode;
use axum_test::TestServer;
use repo::dto::user_dto::UserDto;
use repo::utils::user::user_test_helper;

pub async fn create_user1(client: &TestServer) -> UserDto {
    let create_dto = user_test_helper::get_create_dto1();
    let expected_status_code = StatusCode::OK;
    let expected_user_dto = user_test_helper::get_created_dto1();

    let res = client.post("/api/v1/users").json(&create_dto).await;
    let user_dto: UserDto = res.json();

    assert_eq!(res.status_code(), expected_status_code);
    assert_eq!(user_dto, expected_user_dto);

    user_dto
}

pub async fn create_user2(client: &TestServer) -> UserDto {
    let create_dto = user_test_helper::get_create_dto2();
    let expected_status_code = StatusCode::OK;
    let expected_created_dto = user_test_helper::get_created_dto2();

    let res = client.post("/api/v1/users").json(&create_dto).await;
    let created_dto: UserDto = res.json();

    assert_eq!(res.status_code(), expected_status_code);
    assert_eq!(created_dto, expected_created_dto);
    created_dto
}

pub async fn create_users(client: &TestServer) -> Vec<UserDto> {
    let create_dtos = user_test_helper::get_create_dtos();
    let expected_status_code = StatusCode::OK;
    let mut created_dtos = Vec::new();

    for dto in create_dtos {
        let res = client.post("/api/v1/users").json(&dto).await;
        assert_eq!(res.status_code(), expected_status_code);

        let user_dto: UserDto = res.json();
        created_dtos.push(user_dto);
    }

    created_dtos
}