use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub enum SelectValueEnum {
    TinyNum(i8),
    SmallNum(i16),
    MiddleNum(i32),
    BigNum(i64),
    String(String),
}

impl Default for SelectValueEnum {
    fn default() -> Self {
        Self::TinyNum(0)
    }
}

// 为 SelectValueEnum 实现自定义序列化
impl Serialize for SelectValueEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SelectValueEnum::TinyNum(n) => n.serialize(serializer),
            SelectValueEnum::SmallNum(n) => n.serialize(serializer),
            SelectValueEnum::MiddleNum(n) => n.serialize(serializer),
            SelectValueEnum::BigNum(n) => n.serialize(serializer),
            SelectValueEnum::String(s) => s.serialize(serializer),
        }
    }
}

/// 通用的选择项结构体，用于下拉选择等场景
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct SelectOptionItem {
    /// 显示标签
    pub label: String,

    /// 选择值
    pub value: SelectValueEnum,

    /// 是否禁用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// 分组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,

    /// 子选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<SelectOptionItem>>,
}
