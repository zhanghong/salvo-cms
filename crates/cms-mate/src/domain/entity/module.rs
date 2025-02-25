//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use cms_core::utils::time;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "mate_module")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub editor_type: String,
    pub editor_id: i64,
    pub name: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub sort: i16,
    pub is_enabled: bool,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::kind::Entity")]
    Kind,

    #[sea_orm(has_many = "super::item::Entity")]
    Item,

    #[sea_orm(has_many = "super::morph::Entity")]
    Morph,
}

impl Related<super::kind::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Kind.def()
    }
}

impl Related<super::item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl Related<super::morph::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Morph.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn created_time(&self) -> String {
        time::to_db_time(&self.created_at)
    }

    pub fn updated_time(&self) -> String {
        time::to_db_time(&self.updated_at)
    }
}
