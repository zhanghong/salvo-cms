use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entity::editor::Model;

// ------------------------------------
// 分页查询 VO
// ------------------------------------
#[derive(
    Deserialize, Serialize, FromRedisValue, ToRedisArgs, Debug, Clone, PartialEq, Default, ToSchema,
)]
#[salvo(schema(name = "Core::Base::EditorLoadVO"))]
pub struct EditorLoadVO {
    /// 主键
    #[salvo(schema(required = true, nullable = false, value_type = KnownFormat::Uuid, example = "00000000-0000-0000-0000-000000000000"))]
    pub id: Uuid,

    /// NO
    #[salvo(schema(required = true, nullable = false, value_type = String, max_length=15, example = "administrator"))]
    pub no: String,

    /// 用户名
    #[salvo(schema(required = true, nullable = false, value_type = String, max_length=30, example = "张三"))]
    pub name: String,

    /// 手机号码
    #[salvo(schema(required = true, nullable = false, value_type = String, max_length=11, example = "18021548794"))]
    pub phone: String,

    /// 邮箱
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "zhangsan@example.com"))]
    pub email: String,

    /// 头像URL
    #[salvo(schema(required = false, nullable = false, value_type = KnownFormat::Url, example = "https://www.baidu.com/logo.png"))]
    pub avatar_url: String,
}

impl EditorLoadVO {
    fn from_model_inner(model: &Model) -> Self {
        let avatar_url = model.avatar_url();

        Self {
            id: model.id.clone(),
            no: model.no.to_owned(),
            name: model.name.to_owned(),
            phone: model.phone.to_owned(),
            email: model.email.to_owned(),
            avatar_url,
        }
    }
}

impl From<Model> for EditorLoadVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for EditorLoadVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}
