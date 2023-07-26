pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table_users;
mod m20230725_212714_crate_table_workspaces;
mod m20230725_213206_crate_table_user_workspace;
mod m20230726_034351_crate_table_notes;
mod m20230726_205734_rename_notes_created_by;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_users::Migration),
            Box::new(m20230725_212714_crate_table_workspaces::Migration),
            Box::new(m20230725_213206_crate_table_user_workspace::Migration),
            Box::new(m20230726_034351_crate_table_notes::Migration),
            Box::new(m20230726_205734_rename_notes_created_by::Migration),
        ]
    }
}
