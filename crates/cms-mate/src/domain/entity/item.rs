//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;

use cms_core::{
    domain::{SelectOptionItem, SelectValueEnum},
    utils::time,
};

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "mate_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub editor_type: String,
    pub editor_id: i64,
    pub app_id: i64,
    pub kind_id: i64,
    pub name: String,
    pub title: String,
    pub description: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub introduction: Option<String>,
    pub icon: String,
    pub pc_detail_path: Option<String>,
    pub wap_detail_path: Option<String>,
    pub parent_id: i64,
    pub level: i32,
    pub is_directory: bool,
    pub path: Option<String>,
    pub extends: Option<Json>,
    pub version_no: Option<i32>,
    pub children_count: Option<i16>,
    pub morph_count: Option<i16>,
    pub sort: i16,
    pub is_enabled: bool,
    pub is_deleted: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::app::Entity",
        from = "Column::AppId",
        to = "super::app::Column::Id"
    )]
    App,

    #[sea_orm(
        belongs_to = "super::kind::Entity",
        from = "Column::KindId",
        to = "super::kind::Column::Id"
    )]
    Kind,

    #[sea_orm(has_many = "super::morph::Entity")]
    Morph,
}

impl Related<super::app::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::App.def()
    }
}

impl Related<super::kind::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Kind.def()
    }
}

impl Related<super::morph::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Morph.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn pc_detail_url(&self) -> Option<String> {
        match &self.pc_detail_path {
            Some(path) if path.is_empty() => None,
            Some(path) if !path.starts_with("http://") && !path.starts_with("https://") => {
                let prefix = env::var("MATA_IMATE_URL_PREFIX").ok()?;
                Some(format!("{}{}", prefix, path))
            }
            Some(path) => Some(path.clone()),
            None => None,
        }
    }

    pub fn wap_detail_url(&self) -> Option<String> {
        match &self.wap_detail_path {
            Some(path) if path.is_empty() => None,
            Some(path) if !path.starts_with("http://") && !path.starts_with("https://") => {
                let prefix = env::var("MATA_IMATE_URL_PREFIX").ok()?;
                Some(format!("{}{}", prefix, path))
            }
            Some(path) => Some(path.clone()),
            None => None,
        }
    }

    pub fn created_time(&self) -> Option<String> {
        self.created_at.map(|time| time::to_db_time(&time))
    }

    pub fn updated_time(&self) -> Option<String> {
        self.updated_at.map(|time| time::to_db_time(&time))
    }

    /// Converts the model into a `SelectOptionItem`.
    pub fn to_option_item(&self) -> SelectOptionItem {
        let group = format!("kind-{}-prt-{}", self.kind_id, self.parent_id);
        SelectOptionItem {
            label: self.title.to_string(),
            value: SelectValueEnum::Number(self.id),
            disabled: Some(!self.is_enabled),
            alias: Some(vec![self.name.to_string()]),
            group: Some(group),
            children: None,
            ..Default::default()
        }
    }
}

impl Into<SelectOptionItem> for Model {
    fn into(self) -> SelectOptionItem {
        self.to_option_item()
    }
}

impl Into<SelectOptionItem> for &Model {
    fn into(self) -> SelectOptionItem {
        let group = format!("{}-{}", self.kind_id, self.parent_id);
        SelectOptionItem {
            label: self.title.to_string(),
            value: SelectValueEnum::Number(self.id),
            disabled: Some(!self.is_enabled),
            alias: Some(vec![self.name.to_string()]),
            group: Some(group),
            children: None,
            ..Default::default()
        }
    }
}
