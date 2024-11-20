use mongodb::bson::oid::ObjectId;
use super::error::ApiErrorResponse;
use utoipa::{IntoParams, IntoResponses, ToSchema};
use uuid::Uuid;
use repo::utils::dto::object_id_schema;
use crate::web::dto::user_repo_dto::OneToManyDto;

#[derive(IntoResponses)]
pub enum ApiResponses<'a, T: ToSchema> {
    #[response(status = OK, description = "Successful API call")]
    Ok(#[to_schema] T),

    #[response(status = "4XX", description = "Client error message")]
    ClientError(ApiErrorResponse<'a, String>),

    #[response(status = "5XX", description = "Server error message")]
    ServerError(ApiErrorResponse<'a, String>),
}

#[derive(ToSchema)]
pub struct OpenApiDtoList<T: ToSchema> {
    pub dtos: Vec<T>,
    pub count: u64,
    pub last_taken_entity_number: Option<u64>,
}


#[derive(ToSchema)]
pub struct OpenApiOneToManyDto<O: ToSchema, M: ToSchema>{
    pub one: O,
    pub many: OpenApiDtoList<M>,
}

#[derive(ToSchema)]
pub struct OpenApiOneToOneDto<L: ToSchema, R: ToSchema> {
    pub left: L,
    pub right: R,
}

#[derive(IntoParams)]
pub struct UuidPathParam {
    /// Id of the entity.
    id: Uuid,
}

#[derive(IntoParams)]
pub struct ObjectIdPathParam {
    #[param(schema_with = object_id_schema)]
    /// Id of the entity.
    id: ObjectId
}

