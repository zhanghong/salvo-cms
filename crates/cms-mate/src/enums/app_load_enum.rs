use serde::{Deserialize, Serialize};

/// 懒加载 Model
#[derive(Debug, Clone, PartialEq)]
pub enum AppLoadEnum {
    None,
    Editor,
    Kinds,
    Items,
}

// 实现默认值
impl Default for AppLoadEnum {
    fn default() -> Self {
        Self::None
    }
}

// 序列化
impl Serialize for AppLoadEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AppLoadEnum::None => serializer.serialize_str("none"),
            AppLoadEnum::Editor => serializer.serialize_str("editor"),
            AppLoadEnum::Kinds => serializer.serialize_str("kinds"),
            AppLoadEnum::Items => serializer.serialize_str("items"),
        }
    }
}

// 实现从字符串和数字的反序列化
impl<'de> Deserialize<'de> for AppLoadEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "editor" => Ok(AppLoadEnum::Editor),
            "kinds" => Ok(AppLoadEnum::Kinds),
            "items" => Ok(AppLoadEnum::Items),
            _ => Ok(AppLoadEnum::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AppLoadEnum;
    use serde::{Deserialize, Serialize};

    // 定义一个用于测试的结构体，包含 AppLoadEnum 字段
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct TestStruct {
        load: AppLoadEnum,
    }

    #[test]
    fn test_deserialize_editor() {
        let data = r#"{ "load": "editor" }"#;
        let expected = TestStruct {
            load: AppLoadEnum::Editor,
        };
        let actual: TestStruct = serde_json::from_str(data).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_editor_uppercase() {
        let data = r#"{ "load": "EDITOR" }"#;
        let expected = TestStruct {
            load: AppLoadEnum::Editor,
        };
        let actual: TestStruct = serde_json::from_str(data).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_editor_mixed_case() {
        let data = r#"{ "load": "EdItOr" }"#;
        let expected = TestStruct {
            load: AppLoadEnum::Editor,
        };
        let actual: TestStruct = serde_json::from_str(data).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_kinds() {
        let data = r#"{ "load": "kinds" }"#;
        let expected = TestStruct {
            load: AppLoadEnum::Kinds,
        };
        let actual: TestStruct = serde_json::from_str(data).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_kinds_uppercase() {
        let data = r#"{ "load": "KINDS" }"#;
        let expected = TestStruct {
            load: AppLoadEnum::Kinds,
        };
        let actual: TestStruct = serde_json::from_str(data).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_items() {
        let data = r#"{ "load": "items" }"#;
        let expected = TestStruct {
            load: AppLoadEnum::Items,
        };
        let actual: TestStruct = serde_json::from_str(data).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_items_uppercase() {
        let data = r#"{ "load": "ITEMS" }"#;
        let expected = TestStruct {
            load: AppLoadEnum::Items,
        };
        let actual: TestStruct = serde_json::from_str(data).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_empty_string() {
        let data = r#"{ "load": "" }"#;
        let expected = TestStruct {
            load: AppLoadEnum::None,
        };
        let actual: TestStruct = serde_json::from_str(data).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_invalid_value() {
        let data = r#"{ "load": "invalid" }"#;
        let expected = TestStruct {
            load: AppLoadEnum::None,
        };
        let actual: TestStruct = serde_json::from_str(data).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_serialize_editor() {
        let test = TestStruct {
            load: AppLoadEnum::Editor,
        };
        let expected = r#"{"load":"editor"}"#;
        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_serialize_kinds() {
        let test = TestStruct {
            load: AppLoadEnum::Kinds,
        };
        let expected = r#"{"load":"kinds"}"#;
        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_serialize_items() {
        let test = TestStruct {
            load: AppLoadEnum::Items,
        };
        let expected = r#"{"load":"items"}"#;
        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_serialize_none() {
        let test = TestStruct {
            load: AppLoadEnum::None,
        };
        let expected = r#"{"load":"none"}"#;
        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_default_trait() {
        let default: AppLoadEnum = Default::default();
        assert_eq!(default, AppLoadEnum::None);
    }

    #[test]
    fn test_clone_trait() {
        let a = AppLoadEnum::Editor;
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_debug_trait() {
        let s = format!("{:?}", AppLoadEnum::Items);
        assert_eq!(s, "Items");
    }
}
