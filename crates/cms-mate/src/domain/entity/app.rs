//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use cms_core::{
    domain::{SelectOptionItem, SelectValueEnum},
    utils::time,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "mate_app")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub editor_type: String,
    pub editor_id: i64,
    pub name: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub version_no: Option<i32>,
    pub kind_count: Option<i16>,
    pub sort: i16,
    pub is_enabled: bool,
    pub is_deleted: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
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
    pub fn created_time(&self) -> Option<String> {
        if let Some(time) = (&self).created_at.clone() {
            Some(time::to_db_time(&time))
        } else {
            None
        }
    }

    pub fn updated_time(&self) -> Option<String> {
        if let Some(time) = (&self).updated_at.clone() {
            Some(time::to_db_time(&time))
        } else {
            None
        }
    }
}

impl Into<SelectOptionItem> for Model {
    fn into(self) -> SelectOptionItem {
        SelectOptionItem {
            label: self.title,
            value: SelectValueEnum::Number(self.id),
            disabled: Some(!self.is_enabled),
            alias: Some(vec![self.name]),
            children: None,
            ..Default::default()
        }
    }
}
