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
                            .uuid()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(UserLogin::UserId)
                            .uuid()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserLogin::LoginType)
                            .string_len(15)
                            .not_null()
                            .default("")
                            .comment("登录身份"),
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
                    .name("logins_idx_by_uid")
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
    #[sea_orm(iden = "user_logins")]
    Table,
    Id,
    UserId,
    LoginType,
    ClientIp,
    UserAgent,
    CreatedAt,
}
