use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entity::editor::Model;

/// Editor Load VO
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::From;
    use uuid::Uuid;

    fn mock_model() -> Model {
        let id = Uuid::new_v4();
        Model {
            id,
            no: "admin123".to_string(),
            name: "张三".to_string(),
            phone: "18021548794".to_string(),
            email: "zhangsan@example.com".to_string(),
            avatar_path: "https://www.baidu.com/logo.png".to_string(),
        }
    }

    #[test]
    fn test_from_model_all_fields_filled() {
        let model = mock_model();

        let vo: EditorLoadVO = EditorLoadVO::from(&model);

        assert_eq!(vo.id, model.id);
        assert_eq!(vo.no, "admin123");
        assert_eq!(vo.name, "张三");
        assert_eq!(vo.phone, "18021548794");
        assert_eq!(vo.email, "zhangsan@example.com");
        assert_eq!(vo.avatar_url, "https://www.baidu.com/logo.png");
    }

    #[test]
    fn test_from_model_empty_avatar_url() {
        let mut model = mock_model();
        model.avatar_path = "".to_string();

        let vo: EditorLoadVO = EditorLoadVO::from(model);
        assert_eq!(vo.avatar_url, "");
    }

    #[test]
    fn test_from_model_and_ref_are_consistent() {
        let model = mock_model();

        let vo1: EditorLoadVO = EditorLoadVO::from(model.clone());
        let vo2: EditorLoadVO = EditorLoadVO::from(&model);

        assert_eq!(vo1, vo2);
    }
}
