use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::{
    domain::{SelectOptionItem, vo::EditorLoadVO},
    enums::ViewModeEnum,
};

use crate::domain::entity::app::Model;

// ------------------------------------
// 创建/更新表单选项
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
#[salvo(schema(name = "Mate模块/App/App表单选项VO"))]
pub struct AppFormOptionVO {
    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionItem>>,
}

// ------------------------------------
// 查询表单选项
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
#[salvo(schema(name = "Mate模块/App/App查询选项VO"))]
pub struct AppQueryOptionVO {
    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionItem>>,
}

// ------------------------------------
// 详情 VO
// ------------------------------------
#[derive(
    Debug, Clone, PartialEq, Default, Deserialize, Serialize, ToSchema, FromRedisValue, ToRedisArgs,
)]
#[salvo(schema(name = "Mate模块/App/App主VO"))]
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
    pub description: String,

    /// 图标
    pub icon: String,

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

    /// 编辑用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<EditorLoadVO>,
}

impl AppMasterVO {
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
                vo.created_time = model.created_time().clone();
                vo.updated_time = model.updated_time().clone();
            }
            _ => {}
        }

        vo
    }
}

// ------------------------------------
// 关联 VO
// ------------------------------------
#[derive(
    Debug, Clone, PartialEq, Default, Deserialize, Serialize, ToSchema, FromRedisValue, ToRedisArgs,
)]
#[salvo(schema(name = "Mate模块/App/App关联VO"))]
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
    fn from_model(model: &Model) -> Self {
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
        Self::from_model(&model)
    }
}

impl From<&Model> for AppLoadVO {
    fn from(model: &Model) -> Self {
        Self::from_model(model)
    }
}
