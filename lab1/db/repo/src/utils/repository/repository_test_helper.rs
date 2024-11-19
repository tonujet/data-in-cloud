use serde_json::{json, Value};
use uuid::Uuid;

use entity::{repository, RepositoryType};

use super::{CreateUpdateRepoDto, ResponseRepoDto};



pub fn get_test_create_dto() -> CreateUpdateRepoDto {
    CreateUpdateRepoDto {
        title: "CreateTest".to_string(),
        description: None,
        repo_type: RepositoryType::PRIVATE,
    }
}

pub fn get_test_invalid_create_update_dto() -> CreateUpdateRepoDto {
    CreateUpdateRepoDto {
        title: "1".to_string(),
        description: Some("22".to_string()),
        repo_type: RepositoryType::PRIVATE,
    }
}

pub fn get_test_response_from_invalid_dto() -> Value {
    json!({
        "error_name": "ValidationError",
        "message": {
            "description": [
                {
                    "code": "length",
                    "message": "Must be between 3 and 1000 characters",
                    "params": {
                        "max": 1000,
                        "min": 3,
                        "value": "22"
                    }
                }
            ],
            "title": [
                {
                    "code": "length",
                    "message": "Must be between 3 and 30 characters",
                    "params": {
                        "max": 30,
                        "min": 3,
                        "value": "1"
                    }
                }
            ]
        },
        "status_code": "422",
        "status_code_message": "Unprocessable Entity"
    })
}

pub fn get_test_create_dtos() -> Vec<CreateUpdateRepoDto> {
    let create_dto1 = CreateUpdateRepoDto {
        title: "Test1".to_string(),
        description: None,
        repo_type: RepositoryType::PRIVATE,
    };
    let create_dto2 = CreateUpdateRepoDto {
        title: "Test2".to_string(),
        description: Some("Desc1".to_string()),
        repo_type: RepositoryType::PRIVATE,
    };
    let create_dto3 = CreateUpdateRepoDto {
        title: "Test3".to_string(),
        description: None,
        repo_type: RepositoryType::PUBLIC,
    };
    let create_dto4 = CreateUpdateRepoDto {
        title: "Test4".to_string(),
        description: Some("Desc2".to_string()),
        repo_type: RepositoryType::PUBLIC,
    };
    let create_dto5 = CreateUpdateRepoDto {
        title: "Test5".to_string(),
        description: None,
        repo_type: RepositoryType::PRIVATE,
    };

    vec![
        create_dto1,
        create_dto2,
        create_dto3,
        create_dto4,
        create_dto5,
    ]
}

pub fn get_test_response_from_create_dto() -> ResponseRepoDto {
    ResponseRepoDto {
        id: Default::default(),
        title: "CreateTest".to_string(),
        description: None,
        repo_type: RepositoryType::PRIVATE,
        stars: 0,
        created: Default::default(),
        updated: Default::default(),
    }
}

pub fn get_test_response_from_create_dtos() -> Vec<ResponseRepoDto> {
    let response_dto1 = ResponseRepoDto {
        id: Default::default(),
        title: "Test1".to_string(),
        description: None,
        repo_type: RepositoryType::PRIVATE,
        stars: 0,
        created: Default::default(),
        updated: Default::default(),
    };

    let response_dto2 = ResponseRepoDto {
        id: Default::default(),
        title: "Test2".to_string(),
        description: Some("Desc1".to_string()),
        repo_type: RepositoryType::PRIVATE,
        stars: 0,
        created: Default::default(),
        updated: Default::default(),
    };

    let response_dto3 = ResponseRepoDto {
        id: Default::default(),
        title: "Test3".to_string(),
        description: None,
        repo_type: RepositoryType::PUBLIC,
        stars: 0,
        created: Default::default(),
        updated: Default::default(),
    };

    let response_dto4 = ResponseRepoDto {
        id: Default::default(),
        title: "Test4".to_string(),
        description: Some("Desc2".to_string()),
        repo_type: RepositoryType::PUBLIC,
        stars: 0,
        created: Default::default(),
        updated: Default::default(),
    };
    let response_dto5 = ResponseRepoDto {
        id: Default::default(),
        title: "Test5".to_string(),
        description: None,
        repo_type: RepositoryType::PRIVATE,
        stars: 0,
        created: Default::default(),
        updated: Default::default(),
    };

    vec![
        response_dto1,
        response_dto2,
        response_dto3,
        response_dto4,
        response_dto5,
    ]
}

pub fn get_test_response_from_update_dto() -> ResponseRepoDto {
    ResponseRepoDto {
        id: Default::default(),
        title: "UpdateTest".to_string(),
        description: Some("Updated Description".to_string()),
        repo_type: RepositoryType::PUBLIC,
        stars: 0,
        created: Default::default(),
        updated: Default::default(),
    }
}

pub fn get_test_update_dto() -> CreateUpdateRepoDto {
    CreateUpdateRepoDto {
        title: "UpdateTest".to_string(),
        description: Some("Updated Description".to_string()),
        repo_type: RepositoryType::PUBLIC,
    }
}

pub fn get_test_model() -> repository::Model{
    repository::Model {
        id: Uuid::new_v4(),
        title: "CreateTest".to_string(),
        description: None,
        deleted: false,
        r#type: RepositoryType::PRIVATE,
        stars: 0,
        location: "unknown".to_string(),
        created: Default::default(),
        updated: Default::default(),
    }
}