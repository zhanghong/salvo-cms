use cms_core::domain::vo::EditorLoadVO;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::SelectOptionItem;

use crate::domain::entity::kind::Model;

use super::AppLoadVO;

// ------------------------------------
// 创建/更新表单选项
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct KindFormOptionVO {
    /// App 选项
    pub apps: Vec<SelectOptionItem>,

    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionItem>>,
}

// ------------------------------------
// 查询表单选项
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct KindQueryOptionVO {
    /// App 选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<SelectOptionItem>>,

    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionItem>>,
}

// ------------------------------------
// 详情视图
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct KindMasterVO {
    /// 主键
    pub id: i64,

    /// 编辑用户类型
    #[serde(skip_serializing)]
    pub editor_type: String,

    /// 编辑用户ID
    #[serde(skip_serializing)]
    pub editor_id: i64,

    /// 模块ID
    pub app_id: i64,

    /// 名称
    pub name: String,

    /// 标题
    pub title: String,

    /// 最大层级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level: Option<i8>,

    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// 是否多选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multiple: Option<bool>,

    /// 版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_no: Option<i32>,

    /// 排序编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<i16>,

    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,

    /// 是否可以更新
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_update: Option<bool>,

    /// 是否可以删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,

    /// 创建时间
    pub created_time: String,

    /// 更新时间
    pub updated_time: String,

    /// 详情信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<EditorLoadVO>,

    /// 模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppLoadVO>,
}

impl KindMasterVO {
    fn from_model_inner(model: &Model) -> Self {
        let created_time = model.created_time();
        let updated_time = model.updated_time();

        Self {
            id: model.id,
            editor_type: model.editor_type.to_owned(),
            editor_id: model.editor_id,
            app_id: model.app_id,
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            max_level: Some(model.max_level),
            description: Some(model.description.to_owned()),
            icon: Some(model.icon.to_owned()),
            is_multiple: Some(model.is_multiple),
            version_no: Some(model.version_no),
            sort: Some(model.sort),
            is_enabled: Some(model.is_enabled),
            created_time,
            updated_time,
            ..Default::default()
        }
    }
}

impl From<Model> for KindMasterVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for KindMasterVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}

// ------------------------------------
// 关联视图
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct KindLoadVO {
    /// 主键
    pub id: i64,

    /// 名称
    pub name: String,

    /// 标题
    pub title: String,

    /// 模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppLoadVO>,
}

impl KindLoadVO {
    fn from_model_inner(model: &Model) -> Self {
        Self {
            id: model.id,
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            ..Default::default()
        }
    }
}

impl From<Model> for KindLoadVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for KindLoadVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}
