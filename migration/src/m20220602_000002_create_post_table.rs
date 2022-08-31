use super::m20220602_000001_create_user_table::User;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220602_000001_create_post_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Post::ImageUrl).string())
                    .col(ColumnDef::new(Post::Description).string())
                    .col(ColumnDef::new(Post::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-post_id")
                            .from(Post::Table, Post::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).if_exists().to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Post {
    Table,
    Id,
    Name,
    ImageUrl,
    Description,
    UserId,
}
