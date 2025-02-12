use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserDetail::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserDetail::Id)
                            .big_unsigned()
                            .primary_key()
                            .auto_increment()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::EditorType)
                            .string_len(10)
                            .not_null()
                            .default("system")
                            .comment("编辑类型"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::EditorId)
                            .big_unsigned()
                            .not_null()
                            .default(0)
                            .comment("编辑ID"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::UserId)
                            .big_unsigned()
                            .not_null()
                            .default(0)
                            .unique_key()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::IdentityNo)
                            .string_len(18)
                            .not_null()
                            .default("")
                            .comment("身份证号"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::ProvinceID)
                            .big_unsigned()
                            .not_null()
                            .default(0)
                            .comment("所在省份ID"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::CityID)
                            .big_unsigned()
                            .not_null()
                            .default(0)
                            .comment("所在城市ID"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::DistrictID)
                            .big_unsigned()
                            .not_null()
                            .default(0)
                            .comment("所在区县ID"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::Address)
                            .string_len(150)
                            .not_null()
                            .default("")
                            .comment("详细地址"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::BornOn)
                            .date()
                            .comment("出生日期"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::Emotional)
                            .string_len(50)
                            .not_null()
                            .default("")
                            .comment("情感状态"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::GraduatedFrom)
                            .string_len(80)
                            .not_null()
                            .default("")
                            .comment("毕业院校"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::CompanyName)
                            .string_len(100)
                            .not_null()
                            .default("")
                            .comment("公司名称"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::StaffTitle)
                            .string_len(50)
                            .not_null()
                            .default("")
                            .comment("职位名称"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::Introduction)
                            .string_len(500)
                            .not_null()
                            .default("")
                            .comment("个人简介"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::Honor)
                            .string_len(500)
                            .not_null()
                            .default("")
                            .comment("荣誉奖项"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::Expertises)
                            .string_len(500)
                            .not_null()
                            .default("")
                            .comment("擅长领域"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserDetail::UpdatedAt)
                            .date_time()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserDetail::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserDetail {
    Table,
    Id,
    EditorType,
    EditorId,
    UserId,
    IdentityNo,
    ProvinceID,
    CityID,
    DistrictID,
    Address,
    BornOn,
    Emotional,
    GraduatedFrom,
    CompanyName,
    StaffTitle,
    Introduction,
    Honor,
    Expertises,
    CreatedAt,
    UpdatedAt,
}
