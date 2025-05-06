use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::{Validate, ValidationError};

use cms_core::utils::{deserializer, validate};

// // ------------------------------------
// // 字段验证方法
// // ------------------------------------
fn validate_big_integer_present(num: i64) -> Result<(), ValidationError> {
    validate::numeric_equal_or_greater_than(Some(num), 0)
}

/// Mprph Store Form
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
#[salvo(schema(name = "Mate模块/Morph/Morph表单"))]
pub struct MorphInstanceStoreForm {
    /// 名称
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(
        required(message = "实例类型不能为空"),
        length(min = 1, message = "实例类型不能为空")
    )]
    #[salvo(schema(required = true, nullable = false, value_type = String, min_length = 2, max_length = 30, pattern = r"^[a-zA-Z0-9_-]+$", example = "product_item"))]
    pub instance_type: Option<String>,

    /// 实例ID
    #[serde(default, deserialize_with = "deserializer::string_to_option_i64")]
    #[validate(
        required(message = "实例ID不能为空"),
        custom(function = "validate_big_integer_present", message = "实例ID不能为空")
    )]
    #[salvo(schema(required = true, nullable = false, value_type = i64, minimum = 1, example = 3))]
    pub instance_id: Option<i64>,

    /// 关联Item列表
    #[salvo(schema(required = true, nullable = false, value_type = HashMap<String, String>, example = json!({"item_id": "1", "item_type": "product_item"})))]
    pub items: Option<HashMap<String, String>>,
}
