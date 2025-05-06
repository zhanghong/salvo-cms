use serde::{Deserialize, Serialize};

use cms_core::domain::dto::EditorCurrentDTO;

use crate::domain::form::AppStoreForm;

/// App Store DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct AppStoreDTO {
    /// 主键
    pub id: i64,

    /// 编辑用户
    pub editor: EditorCurrentDTO,

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
