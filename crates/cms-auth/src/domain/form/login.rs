use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
pub struct LoginByPasswordForm {
    /// 登录名
    #[validate(
        required(message = "登录名不能为空"),
        length(min = 1, message = "登录名不能为空")
    )]
    pub username: Option<String>,

    /// 登录密码
    #[validate(
        required(message = "登录密码不能为空"),
        length(min = 1, message = "登录密码不能为空")
    )]
    pub password: Option<String>,
}
