//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4
use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_detail")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub editor_type: String,
    pub editor_id: i64,
    #[sea_orm(unique)]
    pub user_id: i64,
    pub identity_no: String,
    pub province_id: i64,
    pub city_id: i64,
    pub district_id: i64,
    pub address: String,
    pub born_on: Option<NaiveDate>,
    pub emotional: String,
    pub graduated_from: String,
    pub company_name: String,
    pub staff_title: String,
    pub introduction: String,
    pub honor: String,
    pub expertises: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
