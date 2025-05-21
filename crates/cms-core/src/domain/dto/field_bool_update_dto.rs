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
    pub field_value: Option<bool>,
}

impl From<FieldBoolUpdateForm> for FieldBoolUpdateDTO {
    fn from(form: FieldBoolUpdateForm) -> Self {
        Self {
            field_name: form.field_name.to_owned(),
            field_value: form.field_value.clone(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::form::FieldBoolUpdateForm;

    #[test]
    fn test_from_form_with_some_true() {
        let form = FieldBoolUpdateForm {
            field_name: "is_active".to_string(),
            field_value: Some(true),
        };

        let dto: FieldBoolUpdateDTO = form.into();

        assert_eq!(dto.field_name, "is_active");
        assert!(dto.field_value.is_some());
        assert!(dto.field_value.unwrap());
        assert_eq!(dto.id, 0); // 默认 i64 值
        assert_eq!(dto.editor, EditorCurrentDTO::empty()); // 默认结构
    }

    #[test]
    fn test_from_form_with_some_false() {
        let form = FieldBoolUpdateForm {
            field_name: "is_admin".to_string(),
            field_value: Some(false),
        };

        let dto: FieldBoolUpdateDTO = form.into();

        assert_eq!(dto.field_name, "is_admin");
        assert!(dto.field_value.is_some());
        assert!(!dto.field_value.unwrap())
    }

    #[test]
    fn test_from_form_with_none() {
        let form = FieldBoolUpdateForm {
            field_name: "is_deleted".to_string(),
            field_value: None,
        };

        let dto: FieldBoolUpdateDTO = form.into();

        assert_eq!(dto.field_name, "is_deleted");
        assert!(dto.field_value.is_none()); // 默认为 false
    }

    #[test]
    fn test_from_form_with_empty_field_name() {
        let form = FieldBoolUpdateForm {
            field_name: "".to_string(),
            field_value: Some(true),
        };

        let dto: FieldBoolUpdateDTO = form.into();

        assert_eq!(dto.field_name, "");
        assert!(dto.field_value.is_some());
    }
}
