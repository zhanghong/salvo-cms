use cms_core::domain::vo::EditorVO;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::SelectOptionItem;

use crate::domain::entity::kind::Model;
use crate::domain::vo::module::ModuleVO;

// ------------------------------------
// 创建/更新表单选项
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct KindFormOptionVO {
    /// 模块选项
    pub modules: Vec<SelectOptionItem>,

    /// 启用状态
    pub enables: Vec<SelectOptionItem>,
}

// ------------------------------------
// 查询表单选项
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct KindQueryOptionVO {
    /// 模块选项
    pub modules: Option<Vec<SelectOptionItem>>,

    /// 启用状态
    pub enables: Vec<SelectOptionItem>,
}

// ------------------------------------
// 详情视图
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct KindVO {
    /// 主键
    pub id: i64,

    /// 编辑用户类型
    #[serde(skip_serializing)]
    pub editor_type: String,

    /// 编辑用户ID
    #[serde(skip_serializing)]
    pub editor_id: i64,

    /// 模块ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_id: Option<i64>,

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

    /// 是否多选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multiple: Option<bool>,

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

    /// 模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<ModuleVO>,
}

impl KindVO {
    fn from_model_inner(model: &Model) -> Self {
        let created_time = model.created_time();
        let updated_time = model.updated_time();

        Self {
            id: model.id,
            editor_type: model.editor_type.to_owned(),
            editor_id: model.editor_id,
            module_id: Some(model.module_id),
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            description: Some(model.description.to_owned()),
            icon: Some(model.icon.to_owned()),
            is_multiple: Some(model.is_multiple),
            sort: Some(model.sort),
            is_enabled: Some(model.is_enabled),
            created_time,
            updated_time,
            ..Default::default()
        }
    }
}

impl From<Model> for KindVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for KindVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}
