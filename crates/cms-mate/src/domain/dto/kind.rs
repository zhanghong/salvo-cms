use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{form::KindStoreForm, query::KindPaginateQuery},
    enums::KindLoadEnum,
};

// ------------------------------------
// 创建/更新
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KindStoreDTO {
    /// 主键
    pub id: i64,

    /// 编辑用户类型
    pub editor_type: String,

    /// 编辑用户ID
    pub editor_id: i64,

    /// 模块ID
    pub app_id: Option<i64>,

    /// 名称
    pub name: Option<String>,

    /// 标题
    pub title: Option<String>,

    /// 最大层级
    pub max_level: Option<i8>,

    /// 描述
    pub description: Option<String>,

    /// 图标
    pub icon: Option<String>,

    /// 是否多选
    pub is_multiple: Option<bool>,

    /// 排序编号
    pub sort: Option<i16>,

    /// 是否启用
    pub is_enabled: Option<bool>,
}

impl KindStoreDTO {
    fn by_store_form(model: &KindStoreForm) -> Self {
        Self {
            app_id: model.app_id.clone(),
            name: model.name.clone(),
            title: model.title.clone(),
            max_level: model.max_level.clone(),
            description: model.description.clone(),
            icon: model.icon.clone(),
            is_multiple: model.is_multiple.clone(),
            sort: model.sort.clone(),
            is_enabled: model.is_enabled.clone(),
            ..Default::default()
        }
    }
}

impl Default for KindStoreDTO {
    fn default() -> Self {
        Self {
            id: 0,
            editor_type: String::from("system"),
            editor_id: 0,
            app_id: None,
            name: None,
            title: None,
            max_level: None,
            description: None,
            icon: None,
            is_multiple: None,
            sort: None,
            is_enabled: None,
        }
    }
}

impl From<KindStoreForm> for KindStoreDTO {
    fn from(model: KindStoreForm) -> Self {
        Self::by_store_form(&model)
    }
}

impl From<&KindStoreForm> for KindStoreDTO {
    fn from(model: &KindStoreForm) -> Self {
        Self::by_store_form(model)
    }
}

// ------------------------------------
// 查询
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct KindQueryDTO {
    /// 页码
    pub page: Option<u64>,

    /// 每页数量
    pub page_size: Option<u64>,

    /// App ID
    pub app_id: Option<i64>,

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
    pub load_models: Option<Vec<KindLoadEnum>>,
}

impl KindQueryDTO {
    fn from_inner(model: &KindPaginateQuery) -> Self {
        Self {
            page: model.page.clone(),
            page_size: model.page_size.clone(),
            app_id: model.app_id.clone(),
            keyword: model.keyword.clone(),
            title: model.title.clone(),
            is_enabled: model.is_enabled.clone(),
            created_start_time: model.created_start_time.clone(),
            created_end_time: model.created_end_time.clone(),
            ..Default::default()
        }
    }
}

impl From<KindPaginateQuery> for KindQueryDTO {
    fn from(model: KindPaginateQuery) -> Self {
        Self::from_inner(&model)
    }
}

impl From<&KindPaginateQuery> for KindQueryDTO {
    fn from(model: &KindPaginateQuery) -> Self {
        Self::from_inner(model)
    }
}

// ------------------------------------
// 查看
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct KindViewDTO {
    /// 主键
    pub id: i64,

    /// 是否启用
    pub enabled: Option<bool>,

    /// 加载关联数据
    pub load_models: Option<Vec<KindLoadEnum>>,
}
