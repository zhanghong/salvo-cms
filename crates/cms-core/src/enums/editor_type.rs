use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::{SelectOptionItem, SelectValueEnum};

// 会员类型
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub enum EditorTypeEnum {
    None, // 无效值
    Admin,
    Member,
    Guest,
}

// 实现默认值
impl Default for EditorTypeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl EditorTypeEnum {
    pub fn as_value(&self) -> String {
        let str = match self {
            EditorTypeEnum::Admin => "admin",
            EditorTypeEnum::Member => "member",
            EditorTypeEnum::Guest => "guest",
            _ => "",
        };
        str.to_string()
    }

    pub fn as_title(&self) -> String {
        let str = match self {
            EditorTypeEnum::Admin => "管理员",
            EditorTypeEnum::Member => "会员",
            EditorTypeEnum::Guest => "游客",
            _ => "异常",
        };
        str.to_string()
    }

    /// 将枚举转换为选项列表
    pub fn to_option_list() -> Vec<SelectOptionItem> {
        vec![
            EditorTypeEnum::Admin.into(),
            EditorTypeEnum::Member.into(),
            EditorTypeEnum::Guest.into(),
        ]
    }

    /// 将逗号分隔的字符串转换为 EditorTypeEnum 向量
    pub fn from_comma_str(s: &str) -> Vec<Self> {
        s.split(',')
            .map(|s| s.trim()) // 去除可能的空格
            .filter(|s| !s.is_empty()) // 过滤空字符串
            .map(|s| match s.to_lowercase().as_str() {
                "admin" => EditorTypeEnum::Admin,
                "member" => EditorTypeEnum::Member,
                "guest" => EditorTypeEnum::Guest,
                _ => EditorTypeEnum::None,
            })
            .filter(|item| *item != EditorTypeEnum::None)
            .collect()
    }

    /// 将 EditorTypeEnum 向量转换为逗号分隔的字符串，过滤掉 None 值
    pub fn to_comma_str(types: &[Self]) -> String {
        let str = types
            .iter()
            .filter(|&t| *t != EditorTypeEnum::None) // 过滤掉 None 值
            .map(|t| t.as_value()) // 转换为字符串值
            .collect::<Vec<_>>()
            .join(",");
        if str.is_empty() {
            str
        } else {
            format!(",{},", str)
        }
    }
}

// 实现从字符串和数字的反序列化
impl<'de> Deserialize<'de> for EditorTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "admin" => Ok(EditorTypeEnum::Admin),
            "member" => Ok(EditorTypeEnum::Member),
            "guest" => Ok(EditorTypeEnum::Guest),
            _ => Ok(EditorTypeEnum::None),
        }
    }
}

/// 转成 SelectOptionItem
impl Into<SelectOptionItem> for EditorTypeEnum {
    fn into(self) -> SelectOptionItem {
        let value = self.as_value();
        SelectOptionItem {
            label: self.as_title(),
            value: SelectValueEnum::String(value),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_comma_str() {
        // 测试基本功能
        let types = vec![EditorTypeEnum::Admin, EditorTypeEnum::Member];
        assert_eq!(EditorTypeEnum::to_comma_str(&types), "admin,member");

        // 测试包含 None 值
        let types = vec![
            EditorTypeEnum::Admin,
            EditorTypeEnum::None,
            EditorTypeEnum::Member,
        ];
        assert_eq!(EditorTypeEnum::to_comma_str(&types), "admin,member");

        // 测试空向量
        let types: Vec<EditorTypeEnum> = vec![];
        assert_eq!(EditorTypeEnum::to_comma_str(&types), "");

        // 测试全部为 None
        let types = vec![EditorTypeEnum::None, EditorTypeEnum::None];
        assert_eq!(EditorTypeEnum::to_comma_str(&types), "");

        // 测试单个值
        let types = vec![EditorTypeEnum::Admin];
        assert_eq!(EditorTypeEnum::to_comma_str(&types), "admin");
    }

    #[test]
    fn test_bidirectional_conversion() {
        // 测试从字符串到向量再到字符串的转换
        let original = "admin,member";
        let types = EditorTypeEnum::from_comma_str(original);
        let result = EditorTypeEnum::to_comma_str(&types);
        assert_eq!(original, result);

        // 测试从向量到字符串再到向量的转换
        let original = vec![EditorTypeEnum::Admin, EditorTypeEnum::Member];
        let str = EditorTypeEnum::to_comma_str(&original);
        let result = EditorTypeEnum::from_comma_str(&str);
        assert_eq!(original, result);
    }
}
