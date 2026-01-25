use sea_orm_migration::{prelude::*, schema::*};
use sea_orm_migration::prelude::Keyword::CurrentTimestamp;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chefs::Table)
                    .if_not_exists()
                    .col(pk_auto(Chefs::Id))
                    .col(string(Chefs::Name))
                    .col(string(Chefs::ShortInfo))
                    .col(string_null(Chefs::Description))
                    .col(integer(Chefs::Season))
                    .col(string(Chefs::Source))
                    .col(string_null(Chefs::ProfileKey))
                    .col(timestamp(Chefs::CreatedAt).not_null().default(CurrentTimestamp))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chefs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Chefs {
    Table,
    Id,
    Name,
    ShortInfo,
    Description,
    Season,
    Source,
    ProfileKey,
    CreatedAt,
}
