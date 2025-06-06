//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub editor_type: String,
    pub editor_id: Option<Uuid>,
    pub no: String,
    pub name: String,
    pub real_name: String,
    pub nickname: String,
    pub user_types: String,
    pub gender: i16,
    pub phone: String,
    pub avatar_path: String,
    pub email: String,
    pub data_source_id: Option<Uuid>,
    pub password: String,
    pub old_password: String,
    pub salt: String,
    pub password_modified_at: Option<NaiveDateTime>,
    pub attempted_times: i16,
    pub last_attempted_at: Option<NaiveDateTime>,
    pub last_login_id: Option<Uuid>,
    pub last_login_at: Option<NaiveDateTime>,
    pub is_authed: bool,
    pub is_enabled: bool,
    pub is_test: bool,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn avatar_url(&self) -> String {
        self.avatar_path.to_owned()
    }
}
