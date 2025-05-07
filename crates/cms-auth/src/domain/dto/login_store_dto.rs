use serde::{Deserialize, Serialize};

use crate::domain::form::PasswordLoginForm;

/// 用户登录 DTO 结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct LoginStoreDTO {
    // 登录方式
    pub login_type: String,

    /// 登录名
    pub username: Option<String>,

    /// 登录密码
    pub password: Option<String>,

    pub user_agent: String,

    pub client_ip: String,
}

impl LoginStoreDTO {
    pub fn by_password_form(model: &PasswordLoginForm) -> Self {
        Self {
            login_type: "password".to_string(),
            username: model.username.clone(),
            password: model.password.clone(),
            ..Default::default()
        }
    }
}

impl From<PasswordLoginForm> for LoginStoreDTO {
    fn from(model: PasswordLoginForm) -> Self {
        Self::by_password_form(&model)
    }
}

impl From<&PasswordLoginForm> for LoginStoreDTO {
    fn from(model: &PasswordLoginForm) -> Self {
        Self::by_password_form(model)
    }
}
