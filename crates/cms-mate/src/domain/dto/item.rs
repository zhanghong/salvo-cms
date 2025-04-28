use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use cms_core::domain::dto::EditorCurrent;

use crate::{
    domain::{form::ItemStoreForm, query::ItemPaginateQuery},
    enums::ItemLoadEnum,
};

/// Item Store DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ItemStoreDTO {
    /// 主键
    pub id: i64,

    /// 编辑用户
    pub editor: EditorCurrent,

    /// 模块ID
    pub app_id: Option<i64>,

    /// 类型ID
    pub kind_id: Option<i64>,

    /// 名称
    pub name: Option<String>,

    /// 标题
    pub title: Option<String>,

    /// 描述
    pub description: Option<String>,

    /// 介绍
    pub introduction: Option<String>,

    /// 介绍
    pub icon: Option<String>,

    /// PC端封面图片
    pub pc_detail_path: Option<String>,

    /// 手机端封面图片
    pub wap_detail_path: Option<String>,

    /// 父级ID
    pub parent_id: Option<i64>,

    /// 版本号
    pub version_no: Option<i32>,

    /// 排序编号
    pub sort: Option<i16>,

    /// 是否启用
    pub is_enabled: Option<bool>,
}

impl ItemStoreDTO {
    fn by_store_form(model: &ItemStoreForm) -> Self {
        Self {
            app_id: model.app_id.clone(),
            kind_id: model.kind_id.clone(),
            name: model.name.clone(),
            title: model.title.clone(),
            description: model.description.clone(),
            introduction: model.introduction.clone(),
            icon: model.icon.clone(),
            pc_detail_path: model.pc_detail_path.clone(),
            wap_detail_path: model.wap_detail_path.clone(),
            parent_id: model.parent_id.clone(),
            version_no: model.version_no.clone(),
            sort: model.sort.clone(),
            is_enabled: model.is_enabled.clone(),
            ..Default::default()
        }
    }
}

impl From<ItemStoreForm> for ItemStoreDTO {
    fn from(model: ItemStoreForm) -> Self {
        Self::by_store_form(&model)
    }
}

impl From<&ItemStoreForm> for ItemStoreDTO {
    fn from(model: &ItemStoreForm) -> Self {
        Self::by_store_form(model)
    }
}

/// Item 查询 DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ItemQueryDTO {
    /// 编辑用户
    pub editor: EditorCurrent,

    /// 页码
    pub page: u64,

    /// 每页数量
    pub page_size: u64,

    /// App ID
    pub app_id: Option<i64>,

    /// 类型ID
    pub kind_id: Option<i64>,

    /// 父级ID
    pub parent_id: Option<i64>,

    /// 关键字
    pub keyword: Option<String>,

    /// 标题
    pub title: Option<String>,

    /// 启用状态
    pub is_enabled: Option<bool>,

    /// 创建开始时间
    pub created_start_time: Option<NaiveDateTime>,

    /// 创建结束时间
    pub created_end_time: Option<NaiveDateTime>,

    /// 加载关联数据
    pub load_models: Option<Vec<ItemLoadEnum>>,
}

impl ItemQueryDTO {
    fn from_inner(model: &ItemPaginateQuery) -> Self {
        Self {
            page: model.page,
            page_size: model.page_size,
            app_id: model.app_id.clone(),
            kind_id: model.kind_id.clone(),
            parent_id: model.parent_id.clone(),
            keyword: model.keyword.clone(),
            title: model.title.clone(),
            is_enabled: model.is_enabled.clone(),
            created_start_time: model.created_start_time.clone(),
            created_end_time: model.created_end_time.clone(),
            ..Default::default()
        }
    }
}

impl From<ItemPaginateQuery> for ItemQueryDTO {
    fn from(model: ItemPaginateQuery) -> Self {
        Self::from_inner(&model)
    }
}

impl From<&ItemPaginateQuery> for ItemQueryDTO {
    fn from(model: &ItemPaginateQuery) -> Self {
        Self::from_inner(model)
    }
}
