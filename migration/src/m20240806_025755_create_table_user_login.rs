use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserLogin::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserLogin::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(UserLogin::UserId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserLogin::ClientIp)
                            .string_len(15)
                            .not_null()
                            .default("")
                            .comment("Client IP"),
                    )
                    .col(
                        ColumnDef::new(UserLogin::UserAgent)
                            .string_len(200)
                            .not_null()
                            .default("")
                            .comment("User Agent"),
                    )
                    .col(
                        ColumnDef::new(UserLogin::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("login-by-user-id")
                    .table(UserLogin::Table)
                    .col(UserLogin::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserLogin::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserLogin {
    Table,
    Id,
    UserId,
    ClientIp,
    UserAgent,
    CreatedAt,
}
