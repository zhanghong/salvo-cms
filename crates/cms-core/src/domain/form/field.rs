// 验证字符串长度是否在指定范围内
use salvo::oapi::{ToParameters, ToSchema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::{Validate, ValidationError};

use crate::utils::validate::{hash_map_max_length, numeric_greater_than_zero, string_present};

// ------------------------------------
// Field validate
// ------------------------------------
fn validate_skip_id_greater_than_zero(num: i64) -> Result<(), ValidationError> {
    numeric_greater_than_zero(Some(num))
}

fn validate_extends_size(map: &&HashMap<String, String>) -> Result<(), ValidationError> {
    hash_map_max_length::<String, String>(Some(map), 5)
}

// ------------------------------------
// 验证字段值是否唯一
// ------------------------------------
#[derive(
    Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema, ToParameters,
)]
#[salvo(schema(name = "Core模块/Base/FieldValueUniqueForm"))]
pub struct FieldValueUniqueForm {
    /// 字段名
    #[validate(custom(function = "string_present", message = "字段名不能为空"))]
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "name"))]
    pub field_name: Option<String>,

    /// 字段值
    #[validate(custom(function = "string_present", message = "字段值不能为空"))]
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "product_category"))]
    pub field_value: Option<String>,

    /// Model id
    #[validate(custom(
        function = "validate_skip_id_greater_than_zero",
        message = "Skip id 必须大于 0"
    ))]
    #[salvo(schema(required = false, nullable = false, value_type = i64, example = 31))]
    pub skip_id: Option<i64>,

    /// 扩展参数
    #[validate(custom(function = "validate_extends_size", message = "扩展参数必须小于等于 5"))]
    #[salvo(schema(required = false, nullable = false, value_type = HashMap<String, String>, example = json!({"parent_id": "1"})))]
    pub extends: Option<HashMap<String, String>>,
}

// ------------------------------------
// 创建/更新用户
// ------------------------------------
#[derive(
    Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema, ToParameters,
)]
#[salvo(schema(name = "Core模块/Base/FieldBoolUpdateForm"))]
pub struct FieldBoolUpdateForm {
    /// 字段名
    #[validate(custom(function = "string_present", message = "字段名不能为空"))]
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "name"))]
    pub field_name: Option<String>,

    /// 字段值
    #[validate(required(message = "字段值"))]
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "product_category"))]
    pub field_value: Option<bool>,
}
