use cms_core::domain::vo::EditorLoadVO;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::SelectOptionItem;

use crate::domain::entity::app::Model;

// ------------------------------------
// 创建/更新表单选项
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct AppFormOptionVO {
    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionItem>>,
}

// ------------------------------------
// 查询表单选项
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct AppQueryOptionVO {
    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionItem>>,
}

// ------------------------------------
// 详情视图
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct AppMasterVO {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,

    /// 详情信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<EditorLoadVO>,
}

impl AppMasterVO {
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
            created_time: Some(created_time),
            updated_time: Some(updated_time),
            ..Default::default()
        }
    }
}

impl From<Model> for AppMasterVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for AppMasterVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}

// ------------------------------------
// 详情视图
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct AppLoadVO {
    /// 主键
    pub id: i64,

    /// 名称
    pub name: String,

    /// 标题
    pub title: String,

    /// 图标
    pub icon: String,
}

impl AppLoadVO {
    fn from_model_inner(model: &Model) -> Self {
        Self {
            id: model.id,
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            icon: model.icon.to_owned(),
        }
    }
}

impl From<Model> for AppLoadVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for AppLoadVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}
