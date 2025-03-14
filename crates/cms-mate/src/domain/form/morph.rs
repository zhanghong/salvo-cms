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

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
pub struct MorphInstanceStoreForm {
    /// 名称
    #[serde(deserialize_with = "deserializer::string_to_option_trimmed")]
    #[validate(
        required(message = "实例类型不能为空"),
        length(min = 1, message = "实例类型不能为空")
    )]
    pub instance_type: Option<String>,

    /// 实例ID
    #[serde(deserialize_with = "deserializer::string_to_option_i64")]
    #[validate(custom(function = "validate_big_integer_present", message = "实例ID不能为空"))]
    pub instance_id: Option<i64>,

    /// 关联Item列表
    pub items: Option<HashMap<String, String>>,
}
