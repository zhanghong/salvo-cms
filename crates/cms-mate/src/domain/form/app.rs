use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use cms_core::utils::{deserializer, validate};

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
// 创建/更新 App
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
#[salvo(schema(name = "Mate模块/App/App表单"))]
pub struct AppStoreForm {
    /// 名称
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(custom(function = "validate_field_name", message = "模块名称长度为2-20位"))]
    pub name: Option<String>,

    /// 标题
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(custom(function = "validate_field_title", message = "模块标题长度为2-30位"))]
    pub title: Option<String>,

    /// 描述
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(custom(
        function = "validate_field_description",
        message = "模块简介长度不能超过200个字符"
    ))]
    pub description: Option<String>,

    /// 图标
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(custom(
        function = "validate_field_icon",
        message = "模块图标长度不能超过30个字符"
    ))]
    pub icon: Option<String>,

    /// 版本号
    #[serde(default, deserialize_with = "deserializer::string_to_option_i32")]
    pub version_no: Option<i32>,

    /// 排序编号
    #[serde(default, deserialize_with = "deserializer::string_to_option_i16")]
    #[validate(custom(function = "validate_field_sort", message = "排序编号必须在0-9999之间"))]
    pub sort: Option<i16>,

    /// 是否启用
    #[serde(default, deserialize_with = "deserializer::string_to_option_bool")]
    pub is_enabled: Option<bool>,
}
