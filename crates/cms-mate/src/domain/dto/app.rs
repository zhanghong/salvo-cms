use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use cms_core::enums::EditorTypeEnum;

use crate::{
    domain::{form::AppStoreForm, query::AppPaginateQuery},
    enums::AppLoadEnum,
};

// ------------------------------------
// 创建/更新
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct AppStoreDTO {
    /// 主键
    pub id: i64,

    /// 编辑用户类型
    pub editor_type: EditorTypeEnum,

    /// 编辑用户ID
    pub editor_id: i64,

    /// 名称
    pub name: Option<String>,

    /// 标题
    pub title: Option<String>,

    /// 描述
    pub description: Option<String>,

    /// 图标
    pub icon: Option<String>,

    /// 版本号
    pub version_no: Option<i32>,

    /// 排序编号
    pub sort: Option<i16>,

    /// 是否启用
    pub is_enabled: Option<bool>,
}

impl AppStoreDTO {
    fn from_inner(model: &AppStoreForm) -> Self {
        Self {
            name: model.name.clone(),
            title: model.title.clone(),
            description: model.description.clone(),
            icon: model.icon.clone(),
            version_no: model.version_no.clone(),
            sort: model.sort.clone(),
            is_enabled: model.is_enabled.clone(),
            ..Default::default()
        }
    }
}

impl From<AppStoreForm> for AppStoreDTO {
    fn from(model: AppStoreForm) -> Self {
        Self::from_inner(&model)
    }
}

impl From<&AppStoreForm> for AppStoreDTO {
    fn from(model: &AppStoreForm) -> Self {
        Self::from_inner(model)
    }
}

// ------------------------------------
// 查询
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct AppQueryDTO {
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
    pub load_models: Option<Vec<AppLoadEnum>>,
}

impl AppQueryDTO {
    fn from_inner(model: &AppPaginateQuery) -> Self {
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

impl From<AppPaginateQuery> for AppQueryDTO {
    fn from(model: AppPaginateQuery) -> Self {
        Self::from_inner(&model)
    }
}

impl From<&AppPaginateQuery> for AppQueryDTO {
    fn from(model: &AppPaginateQuery) -> Self {
        Self::from_inner(model)
    }
}

// ------------------------------------
// 查看
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct AppViewDTO {
    /// 主键
    pub id: i64,

    /// 是否启用
    pub enabled: Option<bool>,

    /// 加载关联数据
    pub load_models: Option<Vec<AppLoadEnum>>,
}
