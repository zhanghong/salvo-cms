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

/// 通用的选择项结构体，用于下拉选择等场景
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SelectOptionItem {
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
    #[salvo(schema(required = false, nullable = true, value_type = SelectOptionItem))]
    pub children: Option<Vec<SelectOptionItem>>,
}
