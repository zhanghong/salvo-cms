use serde::{Deserialize, Serialize};

use crate::domain::form::UserUpdatePasswordForm;

// ------------------------------------
// 更新密码 DTO
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct UserUpdatePasswordDTO {
    /// 主键
    pub id: i64,

    /// 当前密码
    pub current_password: Option<String>,

    /// 确认密码
    pub confirm_password: String,

    /// 新密码
    pub new_password: String,
}

impl From<UserUpdatePasswordForm> for UserUpdatePasswordDTO {
    fn from(model: UserUpdatePasswordForm) -> Self {
        Self {
            id: 0,
            current_password: model.current_password,
            confirm_password: model.confirm_password.unwrap(),
            new_password: model.new_password.unwrap(),
        }
    }
}
