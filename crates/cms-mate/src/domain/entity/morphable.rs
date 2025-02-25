//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "mate_morphable")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub editor_type: String,
    pub editor_id: i64,
    pub module_id: i64,
    pub kind_id: i64,
    pub item_id: i64,
    pub morphable_type: String,
    pub morphable_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
