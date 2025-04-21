use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MateMorph::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MateMorph::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(MateMorph::EditorType)
                            .string_len(10)
                            .not_null()
                            .default("system")
                            .comment("编辑类型"),
                    )
                    .col(
                        ColumnDef::new(MateMorph::EditorId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("编辑ID"),
                    )
                    .col(
                        ColumnDef::new(MateMorph::AppId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("模块ID"),
                    )
                    .col(
                        ColumnDef::new(MateMorph::KindId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("类型ID"),
                    )
                    .col(
                        ColumnDef::new(MateMorph::ItemId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("项目ID"),
                    )
                    .col(
                        ColumnDef::new(MateMorph::InstanceType)
                            .string_len(30)
                            .not_null()
                            .default("")
                            .comment("关联类型"),
                    )
                    .col(
                        ColumnDef::new(MateMorph::InstanceId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("关联ID"),
                    )
                    .col(
                        ColumnDef::new(MateMorph::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(MateMorph::UpdatedAt)
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
                    .name("mate_morphables_idx_by_app_id_and_kind_id")
                    .table(MateMorph::Table)
                    .col(MateMorph::AppId)
                    .col(MateMorph::KindId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("mate-morph-by-instance")
                    .table(MateMorph::Table)
                    .col(MateMorph::InstanceType)
                    .col(MateMorph::InstanceId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MateMorph::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MateMorph {
    #[sea_orm(iden = "mate_morphes")]
    Table,
    Id,
    EditorType,
    EditorId,
    AppId,
    KindId,
    ItemId,
    InstanceType,
    InstanceId,
    CreatedAt,
    UpdatedAt,
}
