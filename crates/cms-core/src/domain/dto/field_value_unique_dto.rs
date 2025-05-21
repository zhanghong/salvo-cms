use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::form::FieldValueUniqueForm;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct FieldValueUniqueDTO {
    /// 字段名
    pub field_name: String,

    /// 字段值
    pub field_value: String,

    /// Model id
    pub skip_id: Uuid,

    /// 扩展参数
    pub extends: Option<HashMap<String, String>>,
}

impl From<FieldValueUniqueForm> for FieldValueUniqueDTO {
    fn from(form: FieldValueUniqueForm) -> Self {
        let skip_id = match form.skip_id {
            Some(str) => match Uuid::parse_str(&str) {
                Ok(uuid) => uuid,
                Err(_) => Uuid::nil(),
            },
            None => Uuid::nil(),
        };
        Self {
            skip_id,
            field_name: form.field_name.to_owned(),
            field_value: form.field_value.to_owned(),
            extends: form.extends,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_form_with_valid_skip_id() {
        let mut extends = HashMap::new();
        extends.insert("key".to_string(), "value".to_string());

        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let form = FieldValueUniqueForm {
            field_name: "username".to_string(),
            field_value: "john_doe".to_string(),
            skip_id: Some(uuid_str.to_string()),
            extends: Some(extends.clone()),
        };

        let dto: FieldValueUniqueDTO = form.into();

        assert_eq!(dto.field_name, "username");
        assert_eq!(dto.field_value, "john_doe");
        assert_eq!(dto.skip_id.to_string(), uuid_str);
        assert!(dto.extends.is_some());
        assert_eq!(dto.extends.unwrap(), extends);
    }

    #[test]
    fn test_from_form_with_invalid_skip_id() {
        let form = FieldValueUniqueForm {
            field_name: "email".to_string(),
            field_value: "test@example.com".to_string(),
            skip_id: Some("invalid-uuid".to_string()),
            extends: None,
        };

        let dto: FieldValueUniqueDTO = form.into();

        assert_eq!(dto.skip_id, Uuid::nil());
        assert_eq!(dto.field_value, "test@example.com");
        assert!(dto.extends.is_none());
    }

    #[test]
    fn test_from_form_with_no_skip_id() {
        let form = FieldValueUniqueForm {
            field_name: "age".to_string(),
            field_value: "30".to_string(),
            skip_id: None,
            extends: Some(HashMap::new()),
        };

        let dto: FieldValueUniqueDTO = form.into();

        assert_eq!(dto.skip_id, Uuid::nil());
        assert_eq!(dto.field_name, "age");
        assert!(dto.extends.is_some());
        assert!(dto.extends.unwrap().is_empty());
    }

    #[test]
    fn test_from_form_with_empty_skip_id() {
        let form = FieldValueUniqueForm {
            field_name: "name".to_string(),
            field_value: "Alice".to_string(),
            skip_id: Some("".to_string()),
            extends: None,
        };

        let dto: FieldValueUniqueDTO = form.into();

        assert_eq!(dto.skip_id, Uuid::nil());
        assert_eq!(dto.field_value, "Alice");
        assert!(dto.extends.is_none());
    }
}
