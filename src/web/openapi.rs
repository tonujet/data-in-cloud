use mongodb::bson::oid::ObjectId;
use super::error::ApiErrorResponse;
use utoipa::{IntoParams, IntoResponses, ToSchema};
use uuid::Uuid;
use repo::utils::dto::object_id_schema;

#[derive(IntoResponses)]
#[allow(dead_code)]
pub enum ApiResponses<'a, T: ToSchema> {
    #[response(status = OK, description = "Successful API call")]
    Ok(#[to_schema] T),

    #[response(status = "4XX", description = "Client error message")]
    ClientError(ApiErrorResponse<'a, String>),

    #[response(status = "5XX", description = "Server error message")]
    ServerError(ApiErrorResponse<'a, String>),
}

#[derive(IntoParams)]
#[allow(dead_code)]
pub struct UuidPathParam {
    /// Id of the entity.
    id: Uuid,
}

#[derive(IntoParams)]
#[allow(dead_code)]
pub struct ObjectIdPathParam {
    #[param(schema_with = object_id_schema)]
    /// Id of the entity.
    id: ObjectId
}

