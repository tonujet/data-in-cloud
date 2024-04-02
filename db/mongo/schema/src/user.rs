use std::sync::{Arc, Mutex};
use mongodb::bson::doc;
use mongodb::options::{CreateCollectionOptions, CreateIndexOptions, IndexOptions, ValidationAction, ValidationLevel};
use mongodb::IndexModel;
use collection::user::User;


use crate::Scheme;

pub struct UserScheme {}

impl Scheme for UserScheme {
    type Entity = User;

    fn get_collection_name(&self) -> &'static str {
        "users"
    }

    fn get_validation_options(&self) -> CreateCollectionOptions {
        let validator = doc! {
            "$jsonSchema": doc! {
                "bsonType": "object",
                "title": "User object validation",
                "required": vec!["email", "username", "password", "is_public", "created", "updated", "age"],
                "properties": doc! {
                    "email": doc! {
                        "bsonType": "string",
                        "pattern": "^.{3,200}$",
                        "description": "'email' must be a unique string in (3, 200) character limit and is required"
                    },
                    "username": doc! {
                        "bsonType": "string",
                        "pattern": "^.{3,200}$",
                        "description": "'username' must be a unique string in (3, 200) character limit and is required"
                    },
                    "password": doc! {
                        "bsonType": "string",
                        "pattern": "^.{3,200}$",
                        "description": "'password' must be a string in (3, 200) character limit and is required"
                    },
                    "age": doc! {
                        "bsonType": "int",
                        "minimum": 0,
                        "maximum": 255,
                        "description": "'age' must be a int from 0 to 255 and is required"
                    },
                    "is_public": doc! {
                        "bsonType": "bool",
                        "description": "'is_public' must be a boolean and is required"
                    },
                    "created": doc! {
                        "bsonType": "string",
                        "description": "'created' must be an ISO representation of date and is required"
                    },
                    "updated": doc! {
                        "bsonType": "string",
                        "description": "'updated' must be an ISO representation of date and is required"
                    },
                },
            },
        };

        let validation_opts = CreateCollectionOptions::builder()
            .validator(validator)
            .validation_action(Some(ValidationAction::Error))
            .validation_level(Some(ValidationLevel::Moderate))
            .build();

        validation_opts
    }

    fn get_indexes(&self) -> Vec<(IndexModel, impl Into<Option<CreateIndexOptions>>)> {
        let options = IndexOptions::builder().unique(true).build();
        let email_index = IndexModel::builder()
            .keys(doc! {"email": 1})
            .options(options.clone())
            .build();
        let username_index = IndexModel::builder()
            .keys(doc! {"username": 1})
            .options(options)
            .build();

        vec![(email_index, None), (username_index, None)]
    }
    
    fn new() -> Self {
        UserScheme {}
    }
}


