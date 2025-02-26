use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MateKind::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MateKind::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(MateKind::EditorType)
                            .string_len(10)
                            .not_null()
                            .default("system")
                            .comment("编辑类型"),
                    )
                    .col(
                        ColumnDef::new(MateKind::EditorId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("编辑ID"),
                    )
                    .col(
                        ColumnDef::new(MateKind::AppId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("模块ID"),
                    )
                    .col(
                        ColumnDef::new(MateKind::Name)
                            .string_len(30)
                            .not_null()
                            .default("")
                            .comment("名称"),
                    )
                    .col(
                        ColumnDef::new(MateKind::Title)
                            .string_len(30)
                            .not_null()
                            .default("")
                            .comment("标题"),
                    )
                    .col(
                        ColumnDef::new(MateKind::MaxLevel)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("最大层级"),
                    )
                    .col(
                        ColumnDef::new(MateKind::Description)
                            .string_len(200)
                            .not_null()
                            .default("")
                            .comment("描述"),
                    )
                    .col(
                        ColumnDef::new(MateKind::Icon)
                            .string_len(200)
                            .not_null()
                            .default("")
                            .comment("图标"),
                    )
                    .col(
                        ColumnDef::new(MateKind::IsMultiple)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("是否多选"),
                    )
                    .col(
                        ColumnDef::new(MateKind::Sort)
                            .small_integer()
                            .not_null()
                            .default(99)
                            .comment("排序编号"),
                    )
                    .col(
                        ColumnDef::new(MateKind::IsEnabled)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("是否启用"),
                    )
                    .col(
                        ColumnDef::new(MateKind::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("是否删除"),
                    )
                    .col(
                        ColumnDef::new(MateKind::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(MateKind::UpdatedAt)
                            .date_time()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .col(
                        ColumnDef::new(MateKind::DeletedAt)
                            .date_time()
                            .comment("删除时间"),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-by-app-deleted")
                    .table(MateKind::Table)
                    .col(MateKind::AppId)
                    .col(MateKind::IsDeleted)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MateKind::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MateKind {
    Table,
    Id,
    EditorType,
    EditorId,
    AppId,
    Name,
    Title,
    MaxLevel,
    Description,
    Icon,
    IsMultiple,
    Sort,
    IsEnabled,
    IsDeleted,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
