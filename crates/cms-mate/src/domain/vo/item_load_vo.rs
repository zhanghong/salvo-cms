use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use super::app_load_vo::AppLoadVO;
use super::kind_load_vo::KindLoadVO;
use crate::domain::entity::item::Model;

/// Item 关联 VO
#[derive(
    Debug, Clone, PartialEq, Default, Deserialize, Serialize, ToSchema, FromRedisValue, ToRedisArgs,
)]
#[salvo(schema(name = "Mate/Item/ItemLoadVO"))]
pub struct ItemLoadVO {
    /// 主键
    pub id: i64,

    /// App ID
    #[serde(skip_serializing)]
    pub app_id: i64,

    /// 类型ID
    #[serde(skip_serializing)]
    pub kind_id: i64,

    /// 名称
    pub name: String,

    /// 标题
    pub title: String,

    /// 描述
    pub description: String,

    /// 图标
    pub icon: String,

    /// 模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppLoadVO>,

    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<KindLoadVO>,

    /// 子级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ItemLoadVO>>,
}

impl ItemLoadVO {
    fn from_model(model: &Model) -> Self {
        if model.id <= 0 || model.name.is_empty() || model.icon.is_empty() {
            panic!("Invalid Model data: ID must be positive and name/icon cannot be empty");
        }

        Self {
            id: model.id,
            app_id: model.app_id,
            kind_id: model.kind_id,
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            description: model.description.to_owned(),
            icon: model.icon.to_owned(),
            ..Default::default()
        }
    }
}

impl From<Model> for ItemLoadVO {
    fn from(model: Model) -> Self {
        Self::from_model(&model)
    }
}

impl From<&Model> for ItemLoadVO {
    fn from(model: &Model) -> Self {
        Self::from_model(model)
    }
}
