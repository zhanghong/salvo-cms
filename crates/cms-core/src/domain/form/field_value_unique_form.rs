// 验证字符串长度是否在指定范围内
use salvo::oapi::{ToParameters, ToSchema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::{Validate, ValidationError};

use crate::{enums::PrimaryIdEnum, utils::validate_utils::hash_map_max_length};

// ------------------------------------
// Field validate
// ------------------------------------
fn validate_extends_size(map: &&HashMap<String, String>) -> Result<(), ValidationError> {
    hash_map_max_length::<String, String>(Some(map), 5)
}

// ------------------------------------
// 验证字段值是否唯一
// ------------------------------------
#[derive(
    Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema, ToParameters,
)]
#[salvo(schema(name = "Core::Base::FieldValueUniqueForm"))]
pub struct FieldValueUniqueForm {
    /// 字段名
    #[validate(length(min = 2, max = 50, message = "字段名不能为空"))]
    #[salvo(schema(required = true, nullable = false, default, example = "name"))]
    pub field_name: String,

    /// 字段值
    #[validate(length(min = 1, message = "字段值不能为空"))]
    #[salvo(schema(
        required = true,
        nullable = false,
        default,
        example = "product_category"
    ))]
    pub field_value: String,

    /// Model id
    #[salvo(schema(required = false, nullable = false, value_type = PrimaryIdEnum, default, example = "00000000-0000-0000-0000-000000000000"))]
    pub skip_id: PrimaryIdEnum,

    /// 扩展参数
    #[validate(custom(function = "validate_extends_size", message = "扩展参数必须小于等于 5"))]
    #[salvo(schema(required = false, nullable = false, value_type = HashMap<String, String>, default, example = json!({"parent_id": "1"})))]
    pub extends: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    use crate::enums::PrimaryIdEnum;
    use crate::utils::{random_utils::alpha_string, validate_utils::validate_error_hash};

    #[test]
    fn test_valid_form() {
        let mut extends = HashMap::new();
        extends.insert("k1".to_string(), "v1".to_string());

        let form = FieldValueUniqueForm {
            field_name: "name".to_string(),
            field_value: "value".to_string(),
            skip_id: PrimaryIdEnum::Nil,
            extends: Some(extends),
        };

        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_form_field_name() {
        let mut form = FieldValueUniqueForm {
            field_name: "a".to_string(),
            field_value: "value".to_string(),
            skip_id: PrimaryIdEnum::Nil,
            extends: None,
        };

        let result = form.validate();
        let err = result.unwrap_err();
        let map = validate_error_hash(&err);
        assert_eq!(map.get("field_name"), Some(&"字段名不能为空".to_string()));

        form.field_name = alpha_string(51);
        let result = form.validate();
        let err = result.unwrap_err();
        let map = validate_error_hash(&err);
        assert_eq!(map.get("field_name"), Some(&"字段名不能为空".to_string()));
    }

    #[test]
    fn test_form_field_value() {
        let form = FieldValueUniqueForm {
            field_name: "name".to_string(),
            field_value: "".to_string(),
            skip_id: PrimaryIdEnum::Nil,
            extends: None,
        };

        let result = form.validate();
        let err = result.unwrap_err();
        let map = validate_error_hash(&err);
        assert_eq!(map.get("field_value"), Some(&"字段值不能为空".to_string()));
    }

    #[test]
    fn test_form_skip_id() {
        let mut form = FieldValueUniqueForm {
            field_name: "name".to_string(),
            field_value: "value".to_string(),
            skip_id: PrimaryIdEnum::BigInt(5),
            extends: None,
        };

        assert_eq!(form.skip_id, PrimaryIdEnum::BigInt(5));

        let uuid = Uuid::nil();
        form.skip_id = PrimaryIdEnum::Uuid(uuid);
        assert_eq!(form.skip_id, PrimaryIdEnum::Uuid(uuid))
    }

    #[test]
    fn test_form_field_extends() {
        let mut extends = HashMap::new();
        let mut form = FieldValueUniqueForm {
            field_name: "name".to_string(),
            field_value: "value".to_string(),
            skip_id: PrimaryIdEnum::Nil,
            extends: Some(extends.clone()),
        };
        assert!(form.validate().is_ok());

        for i in 0..5 {
            extends.insert(format!("key{}", i), format!("value{}", i));
        }
        form.extends = Some(extends.clone());
        assert!(form.validate().is_ok());

        extends.insert("key6".to_string(), "value6".to_string());
        form.extends = Some(extends.clone());

        let result = form.validate();
        let err = result.unwrap_err();
        let map = validate_error_hash(&err);
        assert_eq!(
            map.get("extends"),
            Some(&"扩展参数必须小于等于 5".to_string())
        );
    }
}
