use serde::{Deserialize, Serialize};

use crate::domain::dto::EditorCurrentDTO;
use crate::domain::form::FieldBoolUpdateForm;

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct FieldBoolUpdateDTO {
    pub id: i64,

    /// 编辑用户
    pub editor: EditorCurrentDTO,

    /// 字段名
    pub field_name: String,

    /// 字段值
    pub field_value: bool,
}

impl From<FieldBoolUpdateForm> for FieldBoolUpdateDTO {
    fn from(form: FieldBoolUpdateForm) -> Self {
        Self {
            field_name: form.field_name.unwrap_or_default(),
            field_value: form.field_value.unwrap_or_default(),
            ..Default::default()
        }
    }
}
