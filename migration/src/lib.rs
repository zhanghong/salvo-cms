pub use sea_orm_migration::prelude::*;

mod m20240806_013138_create_table_user;
mod m20240806_025749_create_table_auth_certificate;
mod m20240806_025755_create_table_user_login;
mod m20240806_025810_create_table_category_item;
mod m20240921_054735_create_table_user_detail;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240806_013138_create_table_user::Migration),
            Box::new(m20240806_025749_create_table_auth_certificate::Migration),
            Box::new(m20240806_025755_create_table_user_login::Migration),
            Box::new(m20240806_025810_create_table_category_item::Migration),
            Box::new(m20240921_054735_create_table_user_detail::Migration),
        ]
    }
}
