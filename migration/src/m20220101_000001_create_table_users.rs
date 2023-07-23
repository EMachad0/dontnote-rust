use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(User::Table)
            .col(
                ColumnDef::new(User::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(User::UUID).uuid().not_null().unique_key())
            .col(ColumnDef::new(User::Name).text().not_null())
            .col(ColumnDef::new(User::Email).text().not_null())
            .col(ColumnDef::new(User::Password).text().not_null())
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::drop().table(User::Table).to_owned();

        manager.drop_table(table).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    UUID,
    Name,
    Email,
    Password,
}
