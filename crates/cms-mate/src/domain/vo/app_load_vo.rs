use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entity::app::Model;

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
