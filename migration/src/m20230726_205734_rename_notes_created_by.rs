use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Note::Table)
            .rename_column(Note::CreatedBy, Note::CreatedById)
            .to_owned();
        manager.alter_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::alter()
            .table(Note::Table)
            .rename_column(Note::CreatedById, Note::CreatedBy)
            .to_owned();
        manager.alter_table(table).await
    }
}

#[derive(Iden)]
enum Note {
    Table,
    CreatedBy,
    CreatedById,
}
