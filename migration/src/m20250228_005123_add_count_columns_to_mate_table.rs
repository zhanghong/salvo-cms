use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(MateApp::Table)
                    .add_column(
                        ColumnDef::new(MateApp::VersionNo)
                            .integer()
                            .not_null()
                            .default(1)
                            .comment("版本号"),
                    )
                    .add_column(
                        ColumnDef::new(MateApp::KindCount)
                            .small_integer()
                            .not_null()
                            .default(0)
                            .comment("有效关联Kind数量"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(MateKind::Table)
                    .add_column(
                        ColumnDef::new(MateKind::VersionNo)
                            .integer()
                            .not_null()
                            .default(1)
                            .comment("版本号"),
                    )
                    .add_column(
                        ColumnDef::new(MateKind::ItemCount)
                            .small_integer()
                            .not_null()
                            .default(0)
                            .comment("有效关联Item数量"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(MateItem::Table)
                    .add_column(
                        ColumnDef::new(MateItem::VersionNo)
                            .integer()
                            .not_null()
                            .default(1)
                            .comment("版本号"),
                    )
                    .add_column(
                        ColumnDef::new(MateItem::ChildrenCount)
                            .small_integer()
                            .not_null()
                            .default(0)
                            .comment("有效关联Children数量"),
                    )
                    .add_column(
                        ColumnDef::new(MateItem::MorphCount)
                            .small_integer()
                            .not_null()
                            .default(0)
                            .comment("有效关联Morph数量"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(MateApp::Table)
                    .drop_column(MateApp::VersionNo)
                    .drop_column(MateApp::KindCount)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(MateKind::Table)
                    .drop_column(MateKind::VersionNo)
                    .drop_column(MateKind::ItemCount)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(MateItem::Table)
                    .drop_column(MateItem::VersionNo)
                    .drop_column(MateItem::ChildrenCount)
                    .drop_column(MateItem::MorphCount)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum MateApp {
    #[sea_orm(iden = "mate_apps")]
    Table,
    VersionNo,
    KindCount,
}

#[derive(DeriveIden)]
enum MateKind {
    #[sea_orm(iden = "mate_kinds")]
    Table,
    VersionNo,
    ItemCount,
}

#[derive(DeriveIden)]
enum MateItem {
    #[sea_orm(iden = "mate_items")]
    Table,
    VersionNo,
    ChildrenCount,
    MorphCount,
}
