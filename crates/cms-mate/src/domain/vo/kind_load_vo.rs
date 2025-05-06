use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entity::kind::Model;

use super::AppLoadVO;

/// Kind 关联 VO
#[derive(
    Debug, Clone, PartialEq, Default, Deserialize, Serialize, ToSchema, FromRedisValue, ToRedisArgs,
)]
#[salvo(schema(name = "Mate模块/Kind/Kind关联VO"))]
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
    fn from_model(model: &Model) -> Self {
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
        Self::from_model(&model)
    }
}

impl From<&Model> for KindLoadVO {
    fn from(model: &Model) -> Self {
        Self::from_model(model)
    }
}
