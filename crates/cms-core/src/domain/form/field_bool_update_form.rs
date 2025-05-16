// 验证字符串长度是否在指定范围内
use salvo::oapi::{ToParameters, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::validate_utils::string_present;

// ------------------------------------
// 更新 Bool 类型字段
// ------------------------------------
#[derive(
    Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema, ToParameters,
)]
#[salvo(schema(name = "Core::Base::FieldBoolUpdateForm"))]
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
