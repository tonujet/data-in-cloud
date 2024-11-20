use sea_orm_migration::prelude::sea_query::extension::postgres::Type;
use sea_orm_migration::prelude::*;

use crate::sea_orm::{EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("repository_type"))
                    .values(RepositoryType::iter())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Repository::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Repository::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Repository::Title).string().not_null())
                    .col(ColumnDef::new(Repository::Description).string())
                    .col(
                        ColumnDef::new(Repository::Deleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Repository::Type)
                            .enumeration(Alias::new("repository_type"), RepositoryType::iter())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Repository::Stars)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Repository::Location).string().not_null())
                    .col(ColumnDef::new(Repository::Created).date_time().not_null())
                    .col(ColumnDef::new(Repository::Updated).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        conn.execute_unprepared("SET TIME ZONE 'Europe/Kiev'")
            .await?;

        conn.execute_unprepared(
            "CREATE OR REPLACE FUNCTION set_created_updated() RETURNS TRIGGER AS $$
                    BEGIN
                        NEW.created = COALESCE(NEW.created, CURRENT_TIMESTAMP);
                        NEW.updated = CURRENT_TIMESTAMP;
                        RETURN NEW;
                    END;
                $$ LANGUAGE plpgsql;",
        )
        .await?;

        // Create trigger to execute the set_created_updated function before insert or update
        conn.execute_unprepared(
            "CREATE TRIGGER set_created_updated_trigger
                    BEFORE INSERT OR UPDATE ON repository
                    FOR EACH ROW
                    EXECUTE FUNCTION set_created_updated();",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute_unprepared("RESET TIME ZONE").await?;

        conn.execute_unprepared(
            "DROP TRIGGER IF EXISTS set_created_updated_trigger ON repository;",
        )
        .await?;

        conn.execute_unprepared("DROP FUNCTION IF EXISTS set_created_updated;")
            .await?;

        manager
            .drop_table(Table::drop().table(Repository::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Alias::new("repository_type")).to_owned())
            .await
    }
}

#[derive(Iden, EnumIter)]
enum RepositoryType {
    #[iden = "PRIVATE"]
    PRIVATE,

    #[iden = "PUBLIC"]
    PUBLIC,
}

#[derive(DeriveIden)]
enum Repository {
    Table,
    Id,
    Title,
    Description,
    Deleted,
    Type,
    Stars,
    Location,
    Created,
    Updated,
}
