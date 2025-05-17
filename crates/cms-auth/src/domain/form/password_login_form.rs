use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 密码登录表单
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
#[salvo(schema(name = "Auth::Login::PasswordLoginForm"))]
pub struct PasswordLoginForm {
    /// 登录名
    #[validate(
        required(message = "登录名不能为空"),
        length(min = 1, message = "登录名不能为空")
    )]
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "zhanghong"))]
    pub username: Option<String>,

    /// 登录密码
    #[validate(
        required(message = "登录密码不能为空"),
        length(min = 1, message = "登录密码不能为空")
    )]
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "123456"))]
    pub password: Option<String>,
}
