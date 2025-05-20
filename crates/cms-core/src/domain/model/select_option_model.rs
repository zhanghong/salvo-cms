use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::enums::SelectValueEnum;

/// 通用的选择项结构体，用于下拉选择等场景
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SelectOptionModel {
    /// 显示标签
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "商品"))]
    pub label: String,

    /// 选择值
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "1"))]
    pub value: SelectValueEnum,

    /// 是否禁用
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = bool, example = false, default = false))]
    pub disabled: Option<bool>,

    /// 分组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = String, example = "product"))]
    pub group: Option<String>,

    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = String, example = "product"))]
    pub alias: Option<Vec<String>>,

    /// 子选项
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = SelectOptionModel))]
    pub children: Option<Vec<SelectOptionModel>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_full_serialization() {
        let option = SelectOptionModel {
            label: "商品".to_string(),
            value: SelectValueEnum::String("1".to_string()),
            disabled: Some(false),
            group: Some("product".to_string()),
            alias: Some(vec!["product".to_string(), "item".to_string()]),
            children: Some(vec![SelectOptionModel {
                label: "子项".to_string(),
                value: SelectValueEnum::Number(2),
                disabled: None,
                group: None,
                alias: None,
                children: None,
            }]),
        };

        let json = serde_json::to_value(&option).unwrap();
        assert_eq!(
            json,
            json!({
                "label": "商品",
                "value": "1",
                "disabled": false,
                "group": "product",
                "alias": ["product", "item"],
                "children": [{
                    "label": "子项",
                    "value": 2
                }]
            })
        );

        // 反序列化测试
        let deserialized: SelectOptionModel = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, option);
    }

    #[test]
    fn test_required_fields_only() {
        let option = SelectOptionModel {
            label: "分类".to_string(),
            value: SelectValueEnum::String("1".to_string()),
            disabled: None,
            group: None,
            alias: None,
            children: None,
        };

        let json = serde_json::to_value(&option).unwrap();
        assert_eq!(json, json!({"label": "分类", "value": "1"}));
    }

    #[test]
    fn test_default_values() {
        let default = SelectOptionModel::default();
        assert_eq!(default.label, "");
        match default.value {
            SelectValueEnum::Number(s) => assert_eq!(s, 0),
            SelectValueEnum::String(s) => assert_eq!(s, ""),
            SelectValueEnum::Str(s) => assert_eq!(s, ""),
        }
    }

    #[test]
    fn test_skip_serializing_optional_fields() {
        let option = SelectOptionModel {
            label: "测试".to_string(),
            value: SelectValueEnum::Str("1"),
            disabled: None,
            group: None,
            alias: None,
            children: None,
        };

        let json = serde_json::to_value(&option).unwrap();
        assert!(!json.as_object().unwrap().contains_key("disabled"));
        assert!(!json.as_object().unwrap().contains_key("group"));
        assert!(!json.as_object().unwrap().contains_key("alias"));
        assert!(!json.as_object().unwrap().contains_key("children"));
    }
}
