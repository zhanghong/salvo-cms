use cms_core::domain::vo::EditorVO;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entity::module::Model;

// ------------------------------------
// 用户详情
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct ModuleVO {
    /// 主键
    pub id: i64,

    /// 编辑用户类型
    #[serde(skip_serializing)]
    pub editor_type: String,

    /// 编辑用户ID
    #[serde(skip_serializing)]
    pub editor_id: i64,

    /// 名称
    pub name: String,

    /// 标题
    pub title: String,

    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// 排序编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<i16>,

    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,

    /// 创建时间
    pub created_time: String,

    /// 更新时间
    pub updated_time: String,

    /// 详情信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<EditorVO>,
}

impl ModuleVO {
    fn from_model_inner(model: &Model) -> Self {
        let created_time = model.created_time();
        let updated_time = model.updated_time();

        Self {
            id: model.id,
            editor_type: model.editor_type.to_owned(),
            editor_id: model.editor_id,
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            description: Some(model.description.to_owned()),
            icon: Some(model.icon.to_owned()),
            sort: Some(model.sort),
            is_enabled: Some(model.is_enabled),
            created_time,
            updated_time,
            ..Default::default()
        }
    }
}

impl From<Model> for ModuleVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for ModuleVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}
