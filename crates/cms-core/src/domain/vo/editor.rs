use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entity::editor::Model;

// ------------------------------------
// 分页查询 VO
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct EditorVO {
    /// 主键
    pub id: i64,

    /// NO
    pub no: String,

    /// 用户名
    pub name: String,

    /// 手机号码
    pub phone: String,

    /// 邮箱
    pub email: String,

    /// 头像URL
    pub avatar_url: String,
}

impl EditorVO {
    fn from_model_inner(model: &Model) -> Self {
        let avatar_url = model.avatar_url();

        Self {
            id: model.id,
            no: model.no.to_owned(),
            name: model.name.to_owned(),
            phone: model.phone.to_owned(),
            email: model.email.to_owned(),
            avatar_url,
        }
    }
}

impl From<Model> for EditorVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for EditorVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}
