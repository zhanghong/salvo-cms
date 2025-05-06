use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use cms_core::domain::dto::EditorCurrent;

use crate::{domain::query::KindPaginateQuery, enums::KindLoadEnum};

/// Kind 查询 DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct KindQueryDTO {
    /// 编辑用户
    pub editor: EditorCurrent,

    /// 页码
    pub page: u64,

    /// 每页数量
    pub page_size: u64,

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
            page: model.page,
            page_size: model.page_size,
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
