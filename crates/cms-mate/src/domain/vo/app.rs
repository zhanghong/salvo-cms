use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::{
    domain::{SelectOptionItem, vo::EditorLoadVO},
    enums::ViewModeEnum,
};

use crate::domain::entity::app::Model;

/// App 表单选项 VO
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
#[salvo(schema(name = "Mate模块/App/App表单选项VO"))]
pub struct AppFormOptionVO {
    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = true, nullable = false, value_type = Vec<SelectOptionItem>, example = json!([{"value":true,"label":"启用"},{"value":false,"label":"禁用"}])))]
    pub enables: Option<Vec<SelectOptionItem>>,
}

/// App 查询表单选项 VO
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
#[salvo(schema(name = "Mate模块/App/App查询选项VO"))]
pub struct AppQueryOptionVO {
    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = true, nullable = false, value_type = Vec<SelectOptionItem>, example = json!([{"value":true,"label":"启用"},{"value":false,"label":"禁用"}])))]
    pub enables: Option<Vec<SelectOptionItem>>,
}

/// App 主 VO
#[derive(
    Debug, Clone, PartialEq, Default, Deserialize, Serialize, ToSchema, FromRedisValue, ToRedisArgs,
)]
#[salvo(schema(name = "Mate模块/App/App主VO"))]
pub struct AppMasterVO {
    /// 主键
    #[salvo(parameter(required = true, nullable = false, minimum = 1, example = 1))]
    pub id: i64,

    /// 编辑用户类型
    #[serde(skip_serializing)]
    #[salvo(parameter(required = false, nullable = false, max_length = 10, example = "admin"))]
    pub editor_type: String,

    /// 编辑用户ID
    #[serde(skip_serializing)]
    #[salvo(parameter(required = false, nullable = false, minimum = 0, example = 1))]
    pub editor_id: i64,

    /// 名称
    #[salvo(parameter(
        required = true,
        nullable = false,
        max_length = 30,
        example = "product"
    ))]
    pub name: String,

    /// 标题
    #[salvo(parameter(required = true, nullable = false, max_length = 30, example = "商品"))]
    pub title: String,

    /// 描述
    #[salvo(parameter(
        required = true,
        nullable = false,
        max_length = 200,
        example = "商品描述..."
    ))]
    pub description: String,

    /// 图标
    #[salvo(schema(
        required = true,
        nullable = false,
        max_length = 30,
        pattern = r"^[a-zA-Z0-9_-]+$",
        example = "icon-product"
    ))]
    pub icon: String,

    /// 版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = false, value_type = i32, minimum = 1, example = 3))]
    pub version_no: Option<i32>,

    /// 排序编号
    #[salvo(schema(required = true, nullable = false, value_type = i16, minimum = 0, maximum = 9999, example = 80, default = 99))]
    pub sort: i16,

    /// 是否启用
    #[salvo(schema(required = false, nullable = true, value_type = bool, example = true, default = true))]
    pub is_enabled: bool,

    /// 是否可以更新
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = false, value_type = bool, example = true, default = true))]
    pub can_update: Option<bool>,

    /// 是否可以删除
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = false, value_type = bool, example = true, default = true))]
    pub can_delete: Option<bool>,

    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = false, value_type = String, format = "yyyy-mm-dd HH:MM:SS", example = "2023-08-10 10:00:00"))]
    pub created_time: Option<String>,

    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = String, format = "yyyy-mm-dd HH:MM:SS", example = "2023-08-10 10:00:00"))]
    pub updated_time: Option<String>,

    /// 编辑用户
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = EditorLoadVO))]
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

/// App 关联 VO
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
