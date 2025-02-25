use serde::{Deserialize, Serialize};

use crate::domain::form::ModuleStoreForm;

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModuleStoreDTO {
    /// 名称
    pub name: Option<String>,

    /// 标题
    pub title: Option<String>,

    /// 描述
    pub description: Option<String>,

    /// 图标
    pub icon: Option<String>,

    /// 排序编号
    pub sort: Option<i64>,

    /// 是否启用
    pub is_enabled: Option<bool>,
}

impl ModuleStoreDTO {
    fn by_store_form(model: &ModuleStoreForm) -> Self {
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
        Self::by_store_form(&model)
    }
}

impl From<&ModuleStoreForm> for ModuleStoreDTO {
    fn from(model: &ModuleStoreForm) -> Self {
        Self::by_store_form(model)
    }
}
