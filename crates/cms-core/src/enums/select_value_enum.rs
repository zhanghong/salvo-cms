use salvo::oapi::ToSchema;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq, Clone, ToSchema)]
pub enum SelectValueEnum {
    Number(i64),
    String(String),
    Str(&'static str),
}

impl Default for SelectValueEnum {
    fn default() -> Self {
        Self::Number(0)
    }
}

impl SelectValueEnum {
    /// 构造一个 Number 类型的值
    pub fn from_number(n: i64) -> Self {
        Self::Number(n)
    }

    /// 构造一个 String 类型的值
    pub fn from_string(s: String) -> Self {
        Self::String(s)
    }

    /// 构造一个 &'static str 类型的值
    pub fn from_static_str(s: &'static str) -> Self {
        Self::Str(s)
    }
}

// 为 SelectValueEnum 实现自定义序列化
impl Serialize for SelectValueEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SelectValueEnum::Number(n) => n.serialize(serializer),
            SelectValueEnum::String(s) => s.serialize(serializer),
            SelectValueEnum::Str(s) => s.serialize(serializer),
        }
    }
}

// 为 SelectValueEnum 实现自定义反序列化
impl<'de> Deserialize<'de> for SelectValueEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 使用 serde_json::Value 作为中间值
        let value = serde_json::Value::deserialize(deserializer)?;

        match value {
            serde_json::Value::String(s) => Ok(SelectValueEnum::String(s)),
            serde_json::Value::Number(n) => n
                .as_i64()
                .map(SelectValueEnum::Number)
                .ok_or_else(|| Error::custom("Number must be an integer")),
            _ => Err(Error::custom("Expected string or number")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectValueEnum;
    use serde::{Deserialize, Serialize};
    use serde_json;

    // 定义一个包装结构体用于测试序列化/反序列化
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Wrapper {
        value: SelectValueEnum,
    }

    #[test]
    fn test_constructors() {
        assert_eq!(SelectValueEnum::from_number(42), SelectValueEnum::Number(42));
        assert_eq!(
            SelectValueEnum::from_string("hello".to_string()),
            SelectValueEnum::String("hello".to_string())
        );
        assert_eq!(
            SelectValueEnum::from_static_str("static_str"),
            SelectValueEnum::Str("static_str")
        );
    }

    #[test]
    fn test_serialize_number() {
        let val = SelectValueEnum::Number(100);
        let json = serde_json::to_value(&val).unwrap();
        assert_eq!(json, serde_json::Value::Number(100.into()));
    }

    #[test]
    fn test_serialize_string() {
        let val = SelectValueEnum::String("hello".to_string());
        let json = serde_json::to_value(&val).unwrap();
        assert_eq!(json, serde_json::Value::String("hello".to_string()));
    }

    #[test]
    fn test_serialize_static_str() {
        let val = SelectValueEnum::Str("world");
        let json = serde_json::to_value(&val).unwrap();
        assert_eq!(json, serde_json::Value::String("world".to_string()));
    }

    #[test]
    fn test_deserialize_string() {
        let json = serde_json::Value::String("test".to_string());
        let wrapper: Wrapper = serde_json::from_value(serde_json::json!({ "value": json })).unwrap();
        assert_eq!(wrapper.value, SelectValueEnum::String("test".to_string()));
    }

    #[test]
    fn test_deserialize_number() {
        let json = serde_json::Value::Number(42.into());
        let wrapper: Wrapper = serde_json::from_value(serde_json::json!({ "value": json })).unwrap();
        assert_eq!(wrapper.value, SelectValueEnum::Number(42));
    }

    #[test]
    fn test_deserialize_float_fails() {
        let json = serde_json::Value::Number(123.45.into());
        let result: Result<Wrapper, _> = serde_json::from_value(serde_json::json!({ "value": json }));
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_boolean_fails() {
        let result: Result<Wrapper, _> = serde_json::from_value(serde_json::json!({ "value": true }));
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_null_fails() {
        let result: Result<Wrapper, _> = serde_json::from_value(serde_json::json!({ "value": null }));
        assert!(result.is_err());
    }

    #[test]
    fn test_default() {
        assert_eq!(SelectValueEnum::default(), SelectValueEnum::Number(0));
    }
}