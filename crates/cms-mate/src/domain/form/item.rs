use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use cms_core::utils::{deserializer, validate};

// // ------------------------------------
// // 字段验证方法
// // ------------------------------------
fn validate_field_name(ptr: &&String) -> Result<(), ValidationError> {
    validate::string_length(ptr.as_str(), true, 5, 30)
}

fn validate_field_title(ptr: &&String) -> Result<(), ValidationError> {
    validate::string_length(ptr.as_str(), true, 2, 30)
}

fn validate_field_description(ptr: &&String) -> Result<(), ValidationError> {
    validate::string_length(ptr.as_str(), false, 0, 200)
}

fn validate_field_introduction(ptr: &&String) -> Result<(), ValidationError> {
    validate::string_length(ptr.as_str(), false, 0, 500)
}

fn validate_field_icon(ptr: &&String) -> Result<(), ValidationError> {
    validate::string_length(ptr.as_str(), false, 0, 30)
}

fn validate_field_upload_path(ptr: &&String) -> Result<(), ValidationError> {
    let value = ptr.as_str();
    let _ = validate::string_length(value, false, 0, 150)?;
    validate::url_address(value, false)
}

fn validate_big_integer_present(num: i64) -> Result<(), ValidationError> {
    validate::numeric_equal_or_greater_than(Some(num), 0)
}

fn validate_field_sort(num: i16) -> Result<(), ValidationError> {
    validate::numeric_range(Some(num), true, 0, 9999)
}

// ------------------------------------
// 创建/更新 Item
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
#[salvo(schema(name = "Mate模块/Item/Item表单"))]
pub struct ItemStoreForm {
    /// 模块ID
    #[serde(default, deserialize_with = "deserializer::string_to_option_i64")]
    #[validate(
        required(message = "模块ID不能为空"),
        custom(function = "validate_big_integer_present", message = "模块ID不能为空")
    )]
    pub app_id: Option<i64>,

    /// 类型ID
    #[serde(default, deserialize_with = "deserializer::string_to_option_i64")]
    #[validate(
        required(message = "模块ID不能为空"),
        custom(function = "validate_big_integer_present", message = "类型ID不能为空")
    )]
    pub kind_id: Option<i64>,

    /// 名称
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(
        required(message = "名称不能为空"),
        custom(function = "validate_field_name", message = "名称长度为5-20位")
    )]
    pub name: Option<String>,

    /// 标题
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(
        required(message = "标题不能为空"),
        custom(function = "validate_field_title", message = "标题长度为2-30位")
    )]
    pub title: Option<String>,

    /// 描述
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(custom(
        function = "validate_field_description",
        message = "描述长度不能超过200个字符"
    ))]
    pub description: Option<String>,

    /// 介绍
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(custom(
        function = "validate_field_introduction",
        message = "介绍长度不能超过500个字符"
    ))]
    pub introduction: Option<String>,

    /// Icon
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(custom(function = "validate_field_icon", message = "图标长度不能超过30个字符"))]
    pub icon: Option<String>,

    /// PC端封面图片
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(custom(
        function = "validate_field_upload_path",
        message = "PC端封面图片路径无效"
    ))]
    pub pc_detail_path: Option<String>,

    /// 手机端封面图片
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(custom(
        function = "validate_field_upload_path",
        message = "手机端封面图片路径无效"
    ))]
    pub wap_detail_path: Option<String>,

    /// 父级ID
    #[serde(default, deserialize_with = "deserializer::string_to_option_i64")]
    pub parent_id: Option<i64>,

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
