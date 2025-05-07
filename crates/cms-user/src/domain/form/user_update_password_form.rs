use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use cms_core::utils::validate_utils;

fn validate_field_password(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate_utils::string_length(str, true, 6, 20)
}

fn validate_field_current_password(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate_utils::string_length(str, false, 6, 20)
}

// ------------------------------------
// 更新密码
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
pub struct UserUpdatePasswordForm {
    /// 当前密码
    #[validate(custom(
        function = "validate_field_current_password",
        message = "当前密码不正确"
    ))]
    pub current_password: Option<String>,

    /// 确认密码
    #[validate(custom(function = "validate_field_password", message = "确认密码长度为6-20位"))]
    pub confirm_password: Option<String>,

    /// 新密码
    #[validate(custom(function = "validate_field_password", message = "登录密码长度为6-20位"))]
    pub new_password: Option<String>,
}
