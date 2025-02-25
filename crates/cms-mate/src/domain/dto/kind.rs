use serde::{Deserialize, Serialize};

use crate::domain::form::KindStoreForm;

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct KindStoreDTO {
    /// 模块ID
    pub module_id: Option<i64>,

    /// 名称
    pub name: Option<String>,

    /// 标题
    pub title: Option<String>,

    /// 描述
    pub description: Option<String>,

    /// 图标
    pub icon: Option<String>,

    /// 是否多选
    pub is_multiple: Option<bool>,

    /// 排序编号
    pub sort: Option<i64>,

    /// 是否启用
    pub is_enabled: Option<bool>,
}

impl KindStoreDTO {
    fn by_store_form(model: &KindStoreForm) -> Self {
        Self {
            module_id: model.module_id.clone(),
            name: model.name.clone(),
            title: model.title.clone(),
            description: model.description.clone(),
            icon: model.icon.clone(),
            is_multiple: model.is_multiple.clone(),
            sort: model.sort.clone(),
            is_enabled: model.is_enabled.clone(),
            ..Default::default()
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
