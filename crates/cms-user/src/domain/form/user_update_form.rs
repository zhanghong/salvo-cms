use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use cms_core::utils::validate_utils;

use super::DetailStoreForm;
use crate::enums::GenderEnum;

// // ------------------------------------
// // 字段验证方法
// // ------------------------------------
fn validate_field_no(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate_utils::string_length(str, false, 5, 15)
}

fn validate_field_name(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate_utils::string_length(str, true, 5, 30)
}

fn validate_field_real_name(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate_utils::string_length(str, true, 2, 30)
}

fn validate_field_nickname(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate_utils::string_length(str, true, 2, 30)
}

fn validate_field_gender(ptr: &&GenderEnum) -> Result<(), ValidationError> {
    let item = (*ptr).clone();
    let flag = match item {
        GenderEnum::Male => true,
        GenderEnum::Female => true,
        GenderEnum::Unknown => true,
        _ => false,
    };
    validate_utils::is_allow_enum_value(flag)
}

fn validate_field_phone(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate_utils::phone_number(str, false)
}

fn validate_field_avatar_path(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    let _ = validate_utils::string_length(str, false, 0, 150)?;
    validate_utils::url_address(str, false)
}

fn validate_field_email(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    let _ = validate_utils::string_length(str, false, 5, 50)?;
    validate_utils::email_address(str, false)
}

// ------------------------------------
// 更新用户
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
#[salvo(schema(example = json!({"page": 1, "page_size": 10})))]
pub struct UserUpdateForm {
    /// 主键
    pub id: Option<i64>,

    /// 头像URL
    #[validate(custom(function = "validate_field_avatar_path", message = "头像URL无效"))]
    pub avatar_path: Option<String>,

    /// 邮箱
    #[validate(custom(function = "validate_field_email", message = "邮箱格式不正确"))]
    pub email: Option<String>,

    /// 性别
    #[validate(custom(function = "validate_field_gender", message = "用户性别不正确"))]
    pub gender: Option<GenderEnum>,

    /// 是否认证
    pub is_authed: Option<bool>,

    /// 是否启用
    pub is_enabled: Option<bool>,

    /// 是否测试账号
    pub is_test: Option<bool>,

    /// 登录名
    #[validate(custom(function = "validate_field_name", message = "登录名长度为5-20位"))]
    pub name: Option<String>,

    /// 昵称
    #[validate(custom(function = "validate_field_nickname", message = "昵称长度为2-30位"))]
    pub nickname: Option<String>,

    /// NO
    #[validate(custom(function = "validate_field_no", message = "用户编号格式不正确"))]
    pub no: Option<String>,

    /// 手机号码
    #[validate(custom(function = "validate_field_phone", message = "手机号码无效"))]
    pub phone: Option<String>,

    /// 真实姓名
    #[validate(custom(
        function = "validate_field_real_name",
        message = "真实姓名长度为2-30位"
    ))]
    pub real_name: Option<String>,

    /// 角色类型
    pub user_types: Option<String>,

    // 详情信息
    pub detail: Option<DetailStoreForm>,
}
