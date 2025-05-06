use serde::{Deserialize, Serialize};

use cms_core::domain::dto::EditorCurrentDTO;

use crate::domain::form::KindStoreForm;

/// Kind Store DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct KindStoreDTO {
    /// 主键
    pub id: i64,

    /// 编辑用户
    pub editor: EditorCurrentDTO,

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

    /// 版本号
    pub version_no: Option<i32>,

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
            version_no: model.version_no.clone(),
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
