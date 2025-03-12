use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::{
    domain::{SelectOptionItem, vo::EditorLoadVO},
    enums::ViewModeEnum,
};

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
#[derive(
    Debug, Clone, PartialEq, Default, Deserialize, Serialize, ToSchema, FromRedisValue, ToRedisArgs,
)]
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
    pub description: String,

    /// 图标
    pub icon: String,

    /// 是否多选
    pub is_multiple: bool,

    /// 版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_no: Option<i32>,

    /// 排序编号
    pub sort: i16,

    /// 是否启用
    pub is_enabled: bool,

    /// 是否可以更新
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_update: Option<bool>,

    /// 是否可以删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,

    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,

    /// 详情信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<EditorLoadVO>,

    /// 模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppLoadVO>,
}

impl KindMasterVO {
    pub fn mode_into(view_enum: &ViewModeEnum, model: &Model) -> Self {
        let mut vo = Self {
            id: model.id,
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            description: model.description.to_owned(),
            icon: model.icon.to_owned(),
            sort: model.sort,
            is_enabled: model.is_enabled,
            ..Default::default()
        };

        match *view_enum {
            ViewModeEnum::ManagerDetail | ViewModeEnum::ManagerList => {
                vo.editor_type = model.editor_type.to_owned();
                vo.editor_id = model.editor_id;
                vo.version_no = model.version_no;
                vo.is_multiple = model.is_multiple;
                vo.created_time = model.created_time();
                vo.updated_time = model.updated_time();
            }
            _ => {}
        }

        vo
    }
}

// ------------------------------------
// 关联视图
// ------------------------------------
#[derive(
    Debug, Clone, PartialEq, Default, Deserialize, Serialize, ToSchema, FromRedisValue, ToRedisArgs,
)]
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
