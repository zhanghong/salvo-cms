// 验证字符串长度是否在指定范围内
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

use crate::utils::validate::string_present;

// ------------------------------------
// 验证字段值是否唯一
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
pub struct FieldValueUniqueForm {
    /// 字段名
    #[validate(custom(function = "string_present", message = "字段名不能为空"))]
    pub field_name: Option<String>,

    /// 字段值
    #[validate(custom(function = "string_present", message = "字段值不能为空"))]
    pub field_value: Option<String>,

    /// Model id
    pub skip_id: Option<i64>,

    /// 扩展参数
    pub extends: Option<HashMap<String, String>>,
}

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
pub struct FieldBoolUpdateForm {
    /// 字段名
    #[validate(custom(function = "string_present", message = "字段名不能为空"))]
    pub field_name: Option<String>,

    /// 字段值
    #[validate(required(message = "字段值不能为空"))]
    pub field_value: Option<bool>,
}
