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
    validate::string_length(str, true, 2, 30)
}

fn validate_field_title(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_length(str, true, 2, 30)
}

fn validate_field_description(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_length(str, false, 0, 200)
}

fn validate_field_icon(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_length(str, false, 0, 30)
}

fn validate_field_sort(num: i16) -> Result<(), ValidationError> {
    validate::numeric_range(Some(num), true, 0, 9999)
}

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
pub struct AppStoreForm {
    /// 名称
    #[validate(custom(function = "validate_field_name", message = "名称长度为2-20位"))]
    pub name: Option<String>,

    /// 标题
    #[validate(custom(function = "validate_field_title", message = "标题长度为2-30位"))]
    pub title: Option<String>,

    /// 描述
    #[validate(custom(
        function = "validate_field_description",
        message = "描述长度不能超过200个字符"
    ))]
    pub description: Option<String>,

    /// 图标
    #[validate(custom(function = "validate_field_icon", message = "图标长度不能超过30个字符"))]
    pub icon: Option<String>,

    /// 版本号
    pub version_no: Option<i32>,

    /// 排序编号
    #[validate(custom(function = "validate_field_sort", message = "排序编号必须在0-9999之间"))]
    pub sort: Option<i16>,

    /// 是否启用
    pub is_enabled: Option<bool>,
}
