use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Workspace::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Workspace::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Workspace::UUID)
                    .uuid()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(Workspace::Title).text().not_null())
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::drop().table(Workspace::Table).to_owned();
        manager.drop_table(table).await
    }
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
    UUID,
    Title,
}
