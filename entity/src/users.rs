//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub editor_type: String,
    pub editor_id: Uuid,
    pub no: String,
    pub name: String,
    pub real_name: String,
    pub nickname: String,
    pub user_types: String,
    pub gender: i16,
    pub phone: String,
    pub avatar_path: String,
    pub email: String,
    pub data_source_id: Uuid,
    pub password: String,
    pub old_password: String,
    pub salt: String,
    pub password_modified_at: Option<DateTime>,
    pub attempted_times: i16,
    pub last_attempted_at: Option<DateTime>,
    pub last_login_id: Uuid,
    pub last_login_at: Option<DateTime>,
    pub is_authed: bool,
    pub is_enabled: bool,
    pub is_test: bool,
    pub is_deleted: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
