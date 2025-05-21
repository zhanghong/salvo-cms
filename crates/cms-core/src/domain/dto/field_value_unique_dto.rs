use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{domain::form::FieldValueUniqueForm, enums::PrimaryIdEnum};

///  Field Value Unique DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct FieldValueUniqueDTO {
    /// 字段名
    pub field_name: String,

    /// 字段值
    pub field_value: String,

    /// Model id
    pub skip_id: PrimaryIdEnum,

    /// 扩展参数
    pub extends: Option<HashMap<String, String>>,
}

impl FieldValueUniqueDTO {
    fn from_form_inner(form: &FieldValueUniqueForm) -> Self {
        Self {
            skip_id: form.skip_id.clone(),
            field_name: form.field_name.to_owned(),
            field_value: form.field_value.to_owned(),
            extends: form.extends.clone(),
        }
    }
}

impl From<FieldValueUniqueForm> for FieldValueUniqueDTO {
    fn from(form: FieldValueUniqueForm) -> Self {
        Self::from_form_inner(&form)
    }
}

impl From<&FieldValueUniqueForm> for FieldValueUniqueDTO {
    fn from(form: &FieldValueUniqueForm) -> Self {
        Self::from_form_inner(form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_from_form_with_valid_skip_id() {
        let mut extends = HashMap::new();
        extends.insert("key".to_string(), "value".to_string());

        let mut form = FieldValueUniqueForm {
            field_name: "username".to_string(),
            field_value: "john_doe".to_string(),
            extends: Some(extends.clone()),
            ..Default::default()
        };
        let dto: FieldValueUniqueDTO = (&form).into();

        assert_eq!(dto.field_name, "username");
        assert_eq!(dto.field_value, "john_doe");
        assert_eq!(dto.skip_id, PrimaryIdEnum::Nil);
        assert!(dto.extends.is_some());
        assert_eq!(dto.extends.unwrap(), extends);

        let uuid = Uuid::new_v4();
        form.skip_id = PrimaryIdEnum::Uuid(uuid.clone());
        let dto: FieldValueUniqueDTO = (&form).into();
        assert_eq!(dto.skip_id.active_uuid_id(), Some(uuid));
        assert!(dto.skip_id.active_int_id().is_none());

        let id = 1024;
        form.skip_id = PrimaryIdEnum::BigInt(id);
        let dto: FieldValueUniqueDTO = form.into();
        assert!(dto.skip_id.active_uuid_id().is_none());
        assert_eq!(dto.skip_id.active_int_id(), Some(id));
    }

    #[test]
    fn test_field_extends() {
        let mut form = FieldValueUniqueForm {
            field_name: "name".to_string(),
            field_value: "Alice".to_string(),
            ..Default::default()
        };

        let dto: FieldValueUniqueDTO = (&form).into();
        assert!(dto.extends.is_none());

        let mut extends = HashMap::new();
        form.extends = Some(extends.clone());
        let dto: FieldValueUniqueDTO = (&form).into();
        assert!(dto.extends.is_some());
        assert!(dto.extends.unwrap().is_empty());

        for i in 0..5 {
            extends.insert(format!("key{}", i), format!("value{}", i));
        }
        form.extends = Some(extends.clone());
        let dto: FieldValueUniqueDTO = form.into();
        assert!(dto.extends.is_some());
        assert_eq!(dto.extends.unwrap(), extends);
    }
}
