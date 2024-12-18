//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Deserialize,
    Serialize,
    Copy,
    async_graphql::Enum,
    utoipa::ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "repository_type")]
pub enum RepositoryType {
    #[sea_orm(string_value = "PRIVATE")]
    PRIVATE,

    #[sea_orm(string_value = "PUBLIC")]
    PUBLIC,
}
