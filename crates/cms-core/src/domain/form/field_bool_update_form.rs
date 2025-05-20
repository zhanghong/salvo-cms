// 验证字符串长度是否在指定范围内
use salvo::oapi::{ToParameters, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(
    Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema, ToParameters,
)]
#[salvo(schema(name = "Core::Base::FieldBoolUpdateForm"))]
pub struct FieldBoolUpdateForm {
    /// 字段名
    #[validate(length(min = 2, max = 50, message = "字段名不能为空"))]
    #[salvo(schema(required = true, nullable = false, example = "name"))]
    pub field_name: String,

    /// 字段值
    #[validate(required(message = "字段值不能为空"))]
    #[salvo(schema(required = true, nullable = false, value_type = bool, example =true))]
    pub field_value: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{random_utils::alpha_string, validate_utils::validate_error_hash};

    #[tokio::test]
    async fn test_field_bool_update_form_validate_success() {
        let form = FieldBoolUpdateForm {
            field_name: "no".to_string(),
            field_value: Some(true),
        };

        let result = form.validate();
        assert!(
            result.is_ok(),
            "Expected validation to succeed, got {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_field_bool_update_form_both_fields_empty() {
        let form = FieldBoolUpdateForm {
            field_name: "".to_string(),
            field_value: None,
        };

        let result = form.validate();
        assert!(
            result.is_err(),
            "Expected validation to fail due to empty field_name"
        );
        let err = result.unwrap_err();
        let map = validate_error_hash(&err);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("field_name"), Some(&"字段名不能为空".to_string()));
        assert_eq!(map.get("field_value"), Some(&"字段值不能为空".to_string()));
    }

    #[tokio::test]
    async fn test_field_bool_update_form_min_size_field_name() {
        let form = FieldBoolUpdateForm {
            field_name: "1".to_string(),
            field_value: Some(true),
        };

        let result = form.validate();
        assert!(
            result.is_err(),
            "Expected validation to fail due to empty field_name"
        );
        let err = result.unwrap_err();
        let map = validate_error_hash(&err);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("field_name"), Some(&"字段名不能为空".to_string()));
    }

    #[tokio::test]
    async fn test_field_bool_update_form_max_size_field_name() {
        let name = alpha_string(51);
        let form = FieldBoolUpdateForm {
            field_name: name,
            field_value: Some(true),
        };

        let result = form.validate();
        assert!(
            result.is_err(),
            "Expected validation to fail due to empty field_name"
        );
        let err = result.unwrap_err();
        let map = validate_error_hash(&err);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("field_name"), Some(&"字段名不能为空".to_string()));
    }
}
