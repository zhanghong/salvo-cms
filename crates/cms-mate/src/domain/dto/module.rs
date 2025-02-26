use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{form::ModuleStoreForm, query::ModulePaginateQuery},
    enums::ModuleLoadEnum,
};

// ------------------------------------
// 创建/更新
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModuleStoreDTO {
    /// 主键
    pub id: Option<i64>,

    /// 名称
    pub name: Option<String>,

    /// 标题
    pub title: Option<String>,

    /// 描述
    pub description: Option<String>,

    /// 图标
    pub icon: Option<String>,

    /// 排序编号
    pub sort: Option<i16>,

    /// 是否启用
    pub is_enabled: Option<bool>,
}

impl ModuleStoreDTO {
    fn form_inner(model: &ModuleStoreForm) -> Self {
        Self {
            name: model.name.clone(),
            title: model.title.clone(),
            description: model.description.clone(),
            icon: model.icon.clone(),
            sort: model.sort.clone(),
            is_enabled: model.is_enabled.clone(),
            ..Default::default()
        }
    }
}

impl From<ModuleStoreForm> for ModuleStoreDTO {
    fn from(model: ModuleStoreForm) -> Self {
        Self::form_inner(&model)
    }
}

impl From<&ModuleStoreForm> for ModuleStoreDTO {
    fn from(model: &ModuleStoreForm) -> Self {
        Self::form_inner(model)
    }
}

// ------------------------------------
// 分页查询
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModuleQueryDTO {
    /// 页码
    pub page: Option<u64>,

    /// 每页数量
    pub page_size: Option<u64>,

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
    pub load_models: Option<Vec<ModuleLoadEnum>>,
}

impl ModuleQueryDTO {
    fn form_inner(model: &ModulePaginateQuery) -> Self {
        Self {
            page: model.page.clone(),
            page_size: model.page_size.clone(),
            keyword: model.keyword.clone(),
            title: model.title.clone(),
            is_enabled: model.is_enabled.clone(),
            created_start_time: model.created_start_time.clone(),
            created_end_time: model.created_end_time.clone(),
            ..Default::default()
        }
    }
}

impl From<ModulePaginateQuery> for ModuleQueryDTO {
    fn from(model: ModulePaginateQuery) -> Self {
        Self::form_inner(&model)
    }
}

impl From<&ModulePaginateQuery> for ModuleQueryDTO {
    fn from(model: &ModulePaginateQuery) -> Self {
        Self::form_inner(model)
    }
}

// ------------------------------------
// 查看
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModuleViewDTO {
    /// 主键
    pub id: i64,

    /// 是否启用
    pub enabled: Option<bool>,

    /// 加载关联数据
    pub load_models: Option<Vec<ModuleLoadEnum>>,
}
