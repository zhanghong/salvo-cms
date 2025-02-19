use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use cms_core::utils::validate;

// // ------------------------------------
// // 字段验证方法
// // ------------------------------------
fn validate_field_name(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_present(str)
}

fn validate_field_password(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_present(str)
}

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
pub struct LoginByPasswordForm {
    /// 登录名
    #[validate(custom(function = "validate_field_name", message = "登录名不能为空"))]
    pub name: Option<String>,

    /// 登录密码
    #[validate(custom(function = "validate_field_password", message = "登录密码不能为空"))]
    pub password: Option<String>,
}
