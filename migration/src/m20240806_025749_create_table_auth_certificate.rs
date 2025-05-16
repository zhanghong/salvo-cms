use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuthCertificate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuthCertificate::Id)
                            .uuid()
                            // .default(Expr::cust("gen_random_uuid()"))
                            .primary_key()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(AuthCertificate::UserType)
                            .string_len(10)
                            .not_null()
                            .default("system")
                            .comment("用户类型"),
                    )
                    .col(
                        ColumnDef::new(AuthCertificate::UserId)
                            .uuid()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(AuthCertificate::AccessToken)
                            .string_len(300)
                            .not_null()
                            .default("")
                            .comment("AccessToken"),
                    )
                    .col(
                        ColumnDef::new(AuthCertificate::AccessExpiredAt)
                            .date_time()
                            .not_null()
                            .comment("AccessToken过期时间"),
                    )
                    .col(
                        ColumnDef::new(AuthCertificate::RefreshToken)
                            .string_len(300)
                            .not_null()
                            .default("")
                            .comment("RefreshToken"),
                    )
                    .col(
                        ColumnDef::new(AuthCertificate::RefreshExpiredAt)
                            .date_time()
                            .not_null()
                            .comment("RefreshToken过期时间"),
                    )
                    .col(
                        ColumnDef::new(AuthCertificate::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(AuthCertificate::UpdatedAt)
                            .date_time()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("crets_idx_by_uid")
                    .table(AuthCertificate::Table)
                    .col(AuthCertificate::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuthCertificate::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AuthCertificate {
    #[sea_orm(iden = "auth_certificates")]
    Table,
    Id,
    UserType,
    UserId,
    AccessToken,
    AccessExpiredAt,
    RefreshToken,
    RefreshExpiredAt,
    CreatedAt,
    UpdatedAt,
}
