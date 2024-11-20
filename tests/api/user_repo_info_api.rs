use axum::http::StatusCode;
use mongodb::bson::oid::ObjectId;
use serde_json::{json, Value};
use serial_test::serial;
use uuid::Uuid;

use collection::user_repo_info::UserRepoInfoOperation;
use ia_11_vorobei_ant::web::dto::user_repo_dto::{OneToManyDto, OneToOneDto};
use repo::dto::DtoList;
use repo::dto::user_repo_info_dto::UserRepoInfoDto;

use crate::common::Setup;
use crate::helpers::user_repo_api_helper;

#[tokio::test]
#[serial]
async fn get_user_repo_info_success() {
    let setup = Setup::new().await;
    let OneToOneDto { left: user, right: repo } = user_repo_api_helper::create_user_and_repo(&setup.client).await;
    let expected_code = StatusCode::OK;

    let endpoint = format!("/api/v1/users/{}/repos/{}", user.id.unwrap(), repo.id);
    setup.client.post(&endpoint).await;
    let info_res = setup.client.get("/api/v1/user-repo-infos").await;
    let info_dto_list: DtoList<UserRepoInfoDto> = info_res.json();
    let info_id = info_dto_list.dtos[0].id.unwrap();
    let expected_dto =
        create_info_dto(info_id, user.id.unwrap(), repo.id, UserRepoInfoOperation::CreateLink);
    let info_res = setup
        .client
        .get(&format!("/api/v1/user-repo-infos/{}", info_id))
        .await;

    assert_eq!(
        info_res.status_code(),
        expected_code,
        "Get response status code doesn't correspond to the desired"
    );
    assert_eq!(
        info_res.json::<UserRepoInfoDto>(),
        expected_dto,
        "Get response body of list doesn't correspond to the desired"
    )
}

#[tokio::test]
#[serial]
async fn get_non_existent_user_repo_info_failure() {
    let setup = Setup::new().await;
    let info_id = ObjectId::new();
    let expected_code = StatusCode::CONFLICT;
    let expected_body = json!({
        "status_code": "409",
        "status_code_message": "Conflict",
        "message": format!("UserRepoInfo with id {info_id} not found"),
        "error_name": "RepositoryError"
    });

    let info_res = setup
        .client
        .get(&format!("/api/v1/user-repo-infos/{}", info_id))
        .await;

    assert_eq!(
        info_res.status_code(),
        expected_code,
        "Get response status code doesn't correspond to the desired"
    );
    assert_eq!(
        info_res.json::<Value>(),
        expected_body,
        "Get response body of list doesn't correspond to the desired"
    );
}

#[tokio::test]
#[serial]
async fn list_two_user_repo_info_success() {
    let setup = Setup::new().await;
    let OneToManyDto { one: user, many: repos } =
        user_repo_api_helper::create_connected_user_and_repos(&setup.client).await;
    let expected_code = StatusCode::OK;
    let take: usize = 2;
    let offset = 2;

    let res = setup
        .client
        .get("/api/v1/user-repo-infos")
        .add_query_param("take", take)
        .add_query_param("offset", offset)
        .await;

    assert_eq!(
        res.status_code(),
        expected_code,
        "Get response status code doesn't correspond to the desired"
    );
    let mut info_dto_list: DtoList<UserRepoInfoDto> = res.json();
    assert_eq!(
        info_dto_list.dtos.len(),
        take,
        "Entity length doesn't correspond to the desired"
    );

    for repo_id in repos.dtos.into_iter().map(|d| d.id).rev().skip(offset).take(take) {
        let info_dto = info_dto_list.dtos.remove(0);

        let expected_dto = UserRepoInfoDto {
            id: info_dto.id,
            user_id: user.id.unwrap(),
            repo_id,
            operation: UserRepoInfoOperation::CreateLink,
            executed_at: Default::default(),
        };

        assert_eq!(
            expected_dto, info_dto,
            "Entity doesn't correspond to the desired"
        )
    }
}

fn create_info_dto(
    id: ObjectId,
    user_id: ObjectId,
    repo_id: Uuid,
    operation: UserRepoInfoOperation,
) -> UserRepoInfoDto {
    UserRepoInfoDto {
        id: Some(id),
        user_id,
        repo_id,
        operation,
        executed_at: Default::default(),
    }
}