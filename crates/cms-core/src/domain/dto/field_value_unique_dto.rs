use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::form::FieldValueUniqueForm;

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct FieldValueUniqueDTO {
    /// 字段名
    pub field_name: String,

    /// 字段值
    pub field_value: String,

    /// Model id
    pub skip_id: Option<Uuid>,

    /// 扩展参数
    pub extends: Option<HashMap<String, String>>,
}

impl From<FieldValueUniqueForm> for FieldValueUniqueDTO {
    fn from(form: FieldValueUniqueForm) -> Self {
        let skip_id = match form.skip_id {
            Some(str) => match Uuid::parse_str(&str) {
                Ok(uuid) => Some(uuid),
                Err(_) => None,
            },
            None => None,
        };
        Self {
            skip_id,
            field_name: form.field_name.unwrap_or_default(),
            field_value: form.field_value.unwrap_or_default(),
            extends: form.extends,
        }
    }
}
