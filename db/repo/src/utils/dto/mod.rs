use mongodb::bson::oid::ObjectId;
use serde::Serializer;
use utoipa::openapi::{Object, ObjectBuilder};

pub fn serialize_option_object_id<S>(
    object_id: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_id {
        Some(ref object_id) => serializer.serialize_some(object_id.to_string().as_str()),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_object_id<S>(object_id: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_some(object_id.to_string().as_str())
}


pub fn object_id_schema() -> Object {
    ObjectBuilder::new()
        .schema_type(utoipa::openapi::schema::Type::String)
        .title(Some("Bson ObjectId"))
        .pattern(Some("^[0-9a-fA-F]{24}$"))
        .description(Some("Object id from bson"))
        .build()
}