use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MateItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MateItem::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(MateItem::EditorType)
                            .string_len(10)
                            .not_null()
                            .default("system")
                            .comment("编辑类型"),
                    )
                    .col(
                        ColumnDef::new(MateItem::EditorId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("编辑ID"),
                    )
                    .col(
                        ColumnDef::new(MateItem::ModuleId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("模块ID"),
                    )
                    .col(
                        ColumnDef::new(MateItem::KindId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("类型ID"),
                    )
                    .col(
                        ColumnDef::new(MateItem::Name)
                            .string_len(30)
                            .not_null()
                            .default("")
                            .comment("名称"),
                    )
                    .col(
                        ColumnDef::new(MateItem::Title)
                            .string_len(30)
                            .not_null()
                            .default("")
                            .comment("标题"),
                    )
                    .col(
                        ColumnDef::new(MateItem::Description)
                            .string_len(200)
                            .not_null()
                            .default("")
                            .comment("描述"),
                    )
                    .col(
                        ColumnDef::new(MateItem::Introduction)
                            .text()
                            .comment("简介"),
                    )
                    .col(
                        ColumnDef::new(MateItem::Icon)
                            .string_len(30)
                            .not_null()
                            .default("")
                            .comment("图标"),
                    )
                    .col(
                        ColumnDef::new(MateItem::PcDetailPath)
                            .string_len(150)
                            .not_null()
                            .default("")
                            .comment("PC详情封面图"),
                    )
                    .col(
                        ColumnDef::new(MateItem::WapDetailPath)
                            .string_len(150)
                            .not_null()
                            .default("")
                            .comment("WAP详情封面图"),
                    )
                    .col(
                        ColumnDef::new(MateItem::ParentId)
                            .big_integer()
                            .not_null()
                            .default(0)
                            .comment("父ID"),
                    )
                    .col(
                        ColumnDef::new(MateItem::Level)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(MateItem::IsDirectory)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("是否为目录"),
                    )
                    .col(
                        ColumnDef::new(MateItem::Path)
                            .string_len(200)
                            .not_null()
                            .default("")
                            .comment("路径"),
                    )
                    .col(ColumnDef::new(MateItem::Extends).json().comment("扩展信息"))
                    .col(
                        ColumnDef::new(MateItem::Sort)
                            .small_integer()
                            .not_null()
                            .default(99)
                            .comment("排序编号"),
                    )
                    .col(
                        ColumnDef::new(MateItem::IsEnabled)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("是否启用"),
                    )
                    .col(
                        ColumnDef::new(MateItem::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("是否删除"),
                    )
                    .col(
                        ColumnDef::new(MateItem::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(MateItem::UpdatedAt)
                            .date_time()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .col(
                        ColumnDef::new(MateItem::DeletedAt)
                            .date_time()
                            .comment("删除时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("mate-by-module-and-kind-deleted")
                    .table(MateItem::Table)
                    .col(MateItem::ModuleId)
                    .col(MateItem::KindId)
                    .col(MateItem::IsDeleted)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MateItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MateItem {
    Table,
    Id,
    EditorType,
    EditorId,
    ModuleId,
    KindId,
    Name,
    Title,
    Description,
    Introduction,
    Icon,
    PcDetailPath,
    WapDetailPath,
    ParentId,
    Level,
    IsDirectory,
    Path,
    Extends,
    Sort,
    IsEnabled,
    IsDeleted,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
