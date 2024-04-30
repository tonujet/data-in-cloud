use std::sync::Arc;
use mongodb::bson::oid::ObjectId;

use collection::user::{TestUserCollection, User};
use collection::MongoCollection;

use crate::dao::user_repo::UserRepository;
use crate::dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};

pub fn get_mock_repo() -> UserRepository {
    let collection: Arc<dyn MongoCollection<User>> = Arc::new(TestUserCollection::new());
    UserRepository::new(collection)
}

pub fn get_create_dto1() -> CreateUserDto {
    CreateUserDto {
        email: "create@gmail.com".to_string(),
        username: "create".to_string(),
        password: "create".to_string(),
        age: 55,
        is_public: true,
    }
}

pub fn get_create_dto2() -> CreateUserDto {
    CreateUserDto {
        email: "create2@gmail.com".to_string(),
        username: "create2".to_string(),
        password: "create2".to_string(),
        age: 11,
        is_public: false,
    }
}


pub fn get_created1() -> User {
    User {
        id: Some(ObjectId::new()),
        email: "create@gmail.com".to_string(),
        username: "create".to_string(),
        password: "create".to_string(),
        age: 55,
        is_public: true,
        deleted: false,
        created: Default::default(),
        updated: Default::default(),
    }
}


pub fn get_created2() -> User {
    User {
        id: Some(ObjectId::new()),
        email: "create2@gmail.com".to_string(),
        username: "create2".to_string(),
        password: "create2".to_string(),
        age: 11,
        is_public: false,
        deleted: false,
        created: Default::default(),
        updated: Default::default(),
    }
}

pub fn get_created_dto1() -> UserDto {
    get_created1().into()
}

pub fn get_created_dto2() -> UserDto {
    get_created2().into()
}

pub fn get_create_dtos() -> Vec<CreateUserDto> {
    vec![
        CreateUserDto {
            email: "email1@gmail.com".to_string(),
            username: "username1".to_string(),
            password: "password1".to_string(),
            age: 11,
            is_public: true,
        },
        CreateUserDto {
            email: "email2@gmail.com".to_string(),
            username: "username2".to_string(),
            password: "password2".to_string(),
            age: 13,
            is_public: false,
        },
        CreateUserDto {
            email: "email3@gmail.com".to_string(),
            username: "username3".to_string(),
            password: "password3".to_string(),
            age: 17,
            is_public: true,
        },
        CreateUserDto {
            email: "email4@gmail.com".to_string(),
            username: "username4".to_string(),
            password: "password4".to_string(),
            age: 33,
            is_public: true,
        },
    ]
}

pub fn get_update_dto() -> UpdateUserDto {
   UpdateUserDto {
       username: "update".to_string(),
       age: 11,
       is_public: true,
   } 
}

pub fn get_updated() -> User {
    let update_dto = get_update_dto();
    let mut created = get_created1();
    created.username = update_dto.username;
    created.age = update_dto.age; 
    created.is_public = update_dto.is_public;
    created
}


pub fn get_updated_dto() -> UserDto {
    get_updated().into()
}

