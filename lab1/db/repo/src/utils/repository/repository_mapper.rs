use entity::repository;
use super::ResponseRepoDto;

pub fn map_entity_to_dto(repo: repository::Model) -> ResponseRepoDto {
    let repository::Model {
        id,
        title,
        description,
        deleted: _deleted,
        r#type: repo_type,
        stars,
        location: _location,
        created,
        updated,
    } = repo;

    ResponseRepoDto {
        id,
        title,
        description,
        repo_type,
        stars: stars as u64,
        created,
        updated,
    }
}

pub fn map_entities_to_dtos(repo: Vec<repository::Model>) -> Vec<ResponseRepoDto> {
    repo.into_iter().map(map_entity_to_dto).collect()
}