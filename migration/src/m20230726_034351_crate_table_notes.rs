use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Note::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Note::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Note::UUID).uuid().not_null().unique_key())
            .col(ColumnDef::new(Note::Title).string().not_null())
            .col(ColumnDef::new(Note::Content).string().not_null())
            .col(
                ColumnDef::new(Note::Color)
                    .string()
                    .not_null()
                    .default("0xffffff"),
            )
            .col(ColumnDef::new(Note::WorkspaceId).integer().not_null())
            .col(ColumnDef::new(Note::CreatedBy).integer().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(Note::Table, Note::WorkspaceId)
                    .to(Workspace::Table, Workspace::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Note::Table, Note::CreatedBy)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::drop().table(Note::Table).to_owned();
        manager.drop_table(table).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Note {
    Table,
    Id,
    UUID,
    Title,
    Content,
    Color,
    WorkspaceId,
    CreatedBy,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
}
