use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use cms_core::utils::{deserializer_utils, validate_utils};

// // ------------------------------------
// // 字段验证方法
// // ------------------------------------
fn validate_field_name(ptr: &&String) -> Result<(), ValidationError> {
    validate_utils::string_length(ptr.as_str(), true, 2, 30)
}

fn validate_field_title(ptr: &&String) -> Result<(), ValidationError> {
    validate_utils::string_length(ptr.as_str(), true, 2, 30)
}

fn validate_field_description(ptr: &&String) -> Result<(), ValidationError> {
    validate_utils::string_length(ptr.as_str(), false, 0, 200)
}

fn validate_field_icon(ptr: &&String) -> Result<(), ValidationError> {
    validate_utils::string_length(ptr.as_str(), false, 0, 30)
}

fn validate_field_sort(num: i16) -> Result<(), ValidationError> {
    validate_utils::numeric_range(Some(num), true, 0, 9999)
}

/// App 表单
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
#[salvo(schema(name = "Mate/App/AppStoreForm"))]
pub struct AppStoreForm {
    /// 名称
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_trimmed"
    )]
    #[validate(
        required(message = "模块名称不能为空"),
        custom(function = "validate_field_name", message = "模块标题长度为2-30位")
    )]
    #[salvo(schema(required = true, nullable = false, value_type = String, min_length = 2, max_length = 30, pattern = r"^[a-zA-Z0-9_-]+$", example = "product"))]
    pub name: Option<String>,

    /// 标题
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_trimmed"
    )]
    #[validate(
        required(message = "模块标题不能为空"),
        custom(function = "validate_field_title", message = "模块标题长度为2-30位")
    )]
    #[salvo(schema(required = true, nullable = false, value_type = String, min_length = 2, max_length = 30, example = "商品"))]
    pub title: Option<String>,

    /// 描述
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_trimmed"
    )]
    #[validate(custom(
        function = "validate_field_description",
        message = "模块简介长度不能超过200个字符"
    ))]
    #[salvo(schema(required = false, nullable = false, value_type = String, max_length = 200, example = "模块简介...."))]
    pub description: Option<String>,

    /// 图标
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_trimmed"
    )]
    #[validate(custom(
        function = "validate_field_icon",
        message = "模块图标长度不能超过30个字符"
    ))]
    #[salvo(schema(required = false, nullable = false, value_type = String, max_length = 30, pattern = r"^[a-zA-Z0-9_-]+$", example = "icon-product"))]
    pub icon: Option<String>,

    /// 版本号
    #[serde(default, deserialize_with = "deserializer_utils::string_to_option_i32")]
    #[salvo(schema(required = false, nullable = false, value_type = i32, minimum = 1, example = 3))]
    pub version_no: Option<i32>,

    /// 排序编号
    #[serde(default, deserialize_with = "deserializer_utils::string_to_option_i16")]
    #[validate(custom(function = "validate_field_sort", message = "排序编号必须在0-9999之间"))]
    #[salvo(schema(required = false, nullable = false, value_type = i16, minimum = 0, maximum = 9999, example = 80, default = 99))]
    pub sort: Option<i16>,

    /// 是否启用
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_bool"
    )]
    #[salvo(schema(required = false, nullable = false, value_type = bool, example = true, default = true))]
    pub is_enabled: Option<bool>,
}


mod tests {
    use super::*;

    #[test]
    fn test_app_store_form_valid() {
        let form = AppStoreForm {
            name: Some("product".to_string()),
            title: Some("商品".to_string()),
            description: Some("这是一个商品模块".to_string()),
            icon: Some("icon-product".to_string()),
            version_no: Some(3),
            sort: Some(80),
            is_enabled: Some(true),
        };

        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_app_store_form_missing_required_fields() {
        let form = AppStoreForm::default();
        let result = form.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err().field_errors();

        assert!(errors.contains_key("name"));
        assert!(errors.contains_key("title"));
    }

    #[test]
    fn test_app_store_form_invalid_name_length() {
        let mut form = AppStoreForm::default();
        form.name = Some("a".to_string());
        form.title = Some("商品".to_string());

        let result = form.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err().field_errors();
        assert_eq!(
            errors.get("name").unwrap()[0].message.as_ref().unwrap(),
            "模块标题长度为2-30位"
        );
    }

    #[test]
    fn test_app_store_form_invalid_title_length() {
        let mut form = AppStoreForm::default();
        form.name = Some("product".to_string());
        form.title = Some("太长了太长了太长了太长了太长了".to_string());

        let result = form.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err().field_errors();
        assert_eq!(
            errors.get("title").unwrap()[0].message.as_ref().unwrap(),
            "模块标题长度为2-30位"
        );
    }

    #[test]
    fn test_app_store_form_invalid_description_length() {
        let mut form = AppStoreForm::default();
        form.name = Some("product".to_string());
        form.title = Some("商品".to_string());
        form.description = Some("x".repeat(201));

        let result = form.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err().field_errors();
        assert_eq!(
            errors.get("description").unwrap()[0].message.as_ref().unwrap(),
            "模块简介长度不能超过200个字符"
        );
    }

    #[test]
    fn test_app_store_form_invalid_icon_length() {
        let mut form = AppStoreForm::default();
        form.name = Some("product".to_string());
        form.title = Some("商品".to_string());
        form.icon = Some("x".repeat(31));

        let result = form.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err().field_errors();
        assert_eq!(
            errors.get("icon").unwrap()[0].message.as_ref().unwrap(),
            "模块图标长度不能超过30个字符"
        );
    }

    #[test]
    fn test_app_store_form_invalid_sort_range() {
        let mut form = AppStoreForm::default();
        form.name = Some("product".to_string());
        form.title = Some("商品".to_string());
        form.sort = Some(10000);

        let result = form.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err().field_errors();
        assert_eq!(
            errors.get("sort").unwrap()[0].message.as_ref().unwrap(),
            "排序编号必须在0-9999之间"
        );
    }
}
