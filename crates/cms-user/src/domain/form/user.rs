use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use cms_core::utils::validate;

use super::DetailStoreForm;
use crate::enums::GenderEnum;

// // ------------------------------------
// // 字段验证方法
// // ------------------------------------
fn validate_field_no(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_length(str, false, 5, 15)
}

fn validate_field_name(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_length(str, true, 5, 30)
}

fn validate_field_realname(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_length(str, true, 2, 30)
}

fn validate_field_nickname(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_length(str, true, 2, 30)
}

fn validate_field_gender(ptr: &&GenderEnum) -> Result<(), ValidationError> {
    let item = (*ptr).clone();
    let flag = match item {
        GenderEnum::Male => true,
        GenderEnum::Female => true,
        GenderEnum::Unknown => true,
        _ => false,
    };
    validate::is_allow_enum_value(flag)
}

fn validate_field_phone(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::phone_number(str, false)
}

fn validate_field_avatar_path(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    let _ = validate::string_length(str, false, 0, 150)?;
    validate::url_address(str, false)
}

fn validate_field_email(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    let _ = validate::string_length(str, false, 5, 50)?;
    validate::email_address(str, false)
}

fn validate_field_password(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_length(str, true, 6, 20)
}

fn validate_field_current_password(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_length(str, false, 6, 20)
}

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
#[salvo(schema(example = json!({
    "avatar_path": "http://demo.avatar.com/10.png", 
    "confirm_password": 123456,
    "data_source_id": 1,
    "email": "test@demo.com",
    "gender": 0,
    "is_authed": true,
    "is_enabled": true,
    "is_test": false,
    "name": "zhanghong",
    "nickname": "laifuzi",
    "no": "1234567890",
    "password": 123456,
    "phone": "13800138000",
    "realname": "real name",
    "user_type": "member"
})))]
pub struct UserCreateForm {
    /// 头像URL
    #[validate(custom(function = "validate_field_avatar_path", message = "头像URL无效"))]
    pub avatar_path: Option<String>,

    /// 确认密码
    #[validate(custom(function = "validate_field_password", message = "确认密码长度为6-20位"))]
    pub confirm_password: Option<String>,

    /// 注册来源
    pub data_source_id: Option<i64>,

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

    /// 登录密码
    #[validate(custom(function = "validate_field_password", message = "登录密码长度为6-20位"))]
    pub password: Option<String>,

    /// 手机号码
    #[validate(custom(function = "validate_field_phone", message = "手机号码无效"))]
    pub phone: Option<String>,

    /// 真实姓名
    #[validate(custom(function = "validate_field_realname", message = "真实姓名长度为2-30位"))]
    pub realname: Option<String>,

    /// 角色类型
    pub user_type: Option<String>,

    // 详情信息
    pub detail: Option<DetailStoreForm>,
}

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
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
    #[validate(custom(function = "validate_field_realname", message = "真实姓名长度为2-30位"))]
    pub realname: Option<String>,

    /// 角色类型
    pub user_type: Option<String>,

    // 详情信息
    pub detail: Option<DetailStoreForm>,
}

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
pub struct UserUpdatePasswordForm {
    /// 主键
    pub id: Option<i64>,

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
