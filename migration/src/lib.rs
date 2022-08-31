pub mod m20220602_000001_create_session_table;
pub mod m20220602_000001_create_user_table;
pub mod m20220602_000002_create_post_table;
pub use sea_orm_migration::prelude::*;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220602_000001_create_user_table::Migration),
            Box::new(m20220602_000001_create_session_table::Migration),
            Box::new(m20220602_000002_create_post_table::Migration)
        ]
    }
}
