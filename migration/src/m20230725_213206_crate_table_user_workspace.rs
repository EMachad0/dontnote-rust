use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(UserWorkspace::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(UserWorkspace::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(UserWorkspace::UUID)
                    .uuid()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(UserWorkspace::Role).enumeration(Role::Table, Role::iter().skip(1)))
            .col(ColumnDef::new(UserWorkspace::UserId).integer().not_null())
            .col(
                ColumnDef::new(UserWorkspace::WorkspaceId)
                    .integer()
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(UserWorkspace::Table, UserWorkspace::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(UserWorkspace::Table, UserWorkspace::WorkspaceId)
                    .to(Workspace::Table, Workspace::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::drop().table(UserWorkspace::Table).to_owned();
        manager.drop_table(table).await
    }
}

#[derive(Iden)]
enum UserWorkspace {
    Table,
    Id,
    UUID,
    Role,
    UserId,
    WorkspaceId,
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

#[derive(Iden, EnumIter)]
enum Role {
    Table,
    Admin,
    User,
}
