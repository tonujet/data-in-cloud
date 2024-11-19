use mongodb::bson::doc;
use mongodb::options::{CreateCollectionOptions, ValidationAction, ValidationLevel};

use collection::user_repo_info::UserRepoInfo;

use crate::Scheme;

pub struct UserRepoInfoScheme {}

impl Scheme for UserRepoInfoScheme {
    type Entity = UserRepoInfo;

    fn get_collection_name(&self) -> &'static str {
        "user_repos_info"
    }

    fn get_validation_options(&self) -> CreateCollectionOptions {
        let validator = doc! {
              "$jsonSchema": doc! {
                  "bsonType": "object",
                  "title": "User object validation",
                  "required": vec!["user_id", "repo_id", "operation", "executed_at"],
                  "properties": doc! {
                      "user_id": doc! {
                          "bsonType": "objectId",
                          "description": "'user_id' must be a unique object ID and is required"
                      },
                      "repo_id": doc! {
                          "bsonType": "binData",
                          "description": "'repo_id' must be stored as UUIDv4 in the format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
                      },
                      "operation": doc! {
                          "bsonType": "string",
                          "pattern": "^.{3,200}$",
                          "description": "'operation' must be a string in (3, 200) character limit and is required"
                      },
                      "executed_at": doc! {
                          "bsonType": "string",
                          "description": "'executed_at' must be an ISO representation of date and is required"
                      },
                  },
              },
          };
        CreateCollectionOptions::builder()
            .validator(validator)
            .validation_action(Some(ValidationAction::Error))
            .validation_level(Some(ValidationLevel::Moderate))
            .build()
    }

    fn new() -> Self {
        UserRepoInfoScheme {}
    }
}
