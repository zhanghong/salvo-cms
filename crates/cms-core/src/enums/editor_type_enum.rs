use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use tracing::warn;

use super::SelectValueEnum;
use crate::domain::model::SelectOptionModel;

// 定义常量字符串
const ADMIN_TITLE: &str = "管理员";
const MEMBER_TITLE: &str = "会员";
const GUEST_TITLE: &str = "游客";
const NONE_TITLE: &str = "无效值";
const ADMIN_VALUE: &str = "admin";
const MEMBER_VALUE: &str = "member";
const GUEST_VALUE: &str = "guest";
const NONE_VALUE: &str = "none";

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
    // 将枚举值转换为字符串值
    pub fn as_value(&self) -> &'static str {
        match self {
            EditorTypeEnum::Admin => ADMIN_VALUE,
            EditorTypeEnum::Member => MEMBER_VALUE,
            EditorTypeEnum::Guest => GUEST_VALUE,
            EditorTypeEnum::None => NONE_VALUE,
        }
    }

    // 将枚举值转换为字符串值
    pub fn string_value(&self) -> String {
        self.as_value().to_string()
    }

    // 将枚举值转换为标题字符串
    pub fn as_title(&self) -> &'static str {
        match self {
            EditorTypeEnum::Admin => ADMIN_TITLE,
            EditorTypeEnum::Member => MEMBER_TITLE,
            EditorTypeEnum::Guest => GUEST_TITLE,
            EditorTypeEnum::None => NONE_TITLE,
        }
    }

    /// 将枚举转换为选项列表
    pub fn to_option_list() -> Vec<SelectOptionModel> {
        vec![
            EditorTypeEnum::Admin,
            EditorTypeEnum::Member,
            EditorTypeEnum::Guest,
        ]
        .into_iter()
        .map(|e| e.into())
        .collect()
    }

    // 从字符串值转换为枚举值
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => EditorTypeEnum::Admin,
            "member" => EditorTypeEnum::Member,
            "guest" => EditorTypeEnum::Guest,
            _ => {
                warn!("Invalid input for EditorTypeEnum: {}", s); // 增加日志记录
                EditorTypeEnum::None
            }
        }
    }

    /// 从逗号分隔的字符串转换为 EditorTypeEnum 向量
    pub fn from_comma_str(s: &str) -> Vec<Self> {
        s.split(',')
            .map(|s| s.trim()) // 去除可能的空格
            .filter(|s| !s.is_empty()) // 过滤空字符串
            .map(|s| {
                let lower = s.to_lowercase(); // 提前转换为小写
                match lower.as_str() {
                    "admin" => EditorTypeEnum::Admin,
                    "member" => EditorTypeEnum::Member,
                    "guest" => EditorTypeEnum::Guest,
                    _ => {
                        warn!("Invalid item in comma-separated string: {}", s); // 增加日志记录
                        EditorTypeEnum::None
                    }
                }
            })
            .filter(|item| *item != EditorTypeEnum::None)
            .collect()
    }

    /// 将 EditorTypeEnum 向量转换为逗号分隔的字符串，过滤掉 None 值
    pub fn to_comma_str(types: &[Self]) -> String {
        types
            .iter()
            .filter(|&t| *t != EditorTypeEnum::None) // 过滤掉 None 值
            .map(|t| t.as_value()) // 转换为字符串值
            .collect::<Vec<_>>() // 收集到临时向量
            .join(",") // 直接拼接
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
            _ => {
                warn!("Invalid value during deserialization: {}", s); // 增加日志记录
                Ok(EditorTypeEnum::None)
            }
        }
    }
}

/// 转成 SelectOptionModel
impl Into<SelectOptionModel> for EditorTypeEnum {
    fn into(self) -> SelectOptionModel {
        let value = self.as_value();
        SelectOptionModel {
            label: self.as_title().to_string(),
            value: SelectValueEnum::Str(value),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_value() {
        assert_eq!(EditorTypeEnum::Admin.as_value(), "admin");
        assert_eq!(EditorTypeEnum::Member.as_value(), "member");
        assert_eq!(EditorTypeEnum::Guest.as_value(), "guest");
        assert_eq!(EditorTypeEnum::None.as_value(), "none");
    }

    #[test]
    fn test_string_value() {
        assert_eq!(EditorTypeEnum::Admin.string_value(), "admin".to_string());
        assert_eq!(EditorTypeEnum::Member.string_value(), "member".to_string());
        assert_eq!(EditorTypeEnum::Guest.string_value(), "guest".to_string());
        assert_eq!(EditorTypeEnum::None.string_value(), "none".to_string());
    }

    #[test]
    fn test_as_title() {
        assert_eq!(EditorTypeEnum::Admin.as_title(), "管理员");
        assert_eq!(EditorTypeEnum::Member.as_title(), "会员");
        assert_eq!(EditorTypeEnum::Guest.as_title(), "游客");
        assert_eq!(EditorTypeEnum::None.as_title(), "无效值");
    }

    #[test]
    fn test_to_option_list() {
        let options = EditorTypeEnum::to_option_list();
        assert_eq!(options.len(), 3);
        assert_eq!(options[0].label, "管理员");
        assert_eq!(options[0].value, SelectValueEnum::Str("admin"));
        assert_eq!(options[1].label, "会员");
        assert_eq!(options[1].value, SelectValueEnum::Str("member"));
        assert_eq!(options[2].label, "游客");
        assert_eq!(options[2].value, SelectValueEnum::Str("guest"));
    }

    #[test]
    fn test_from_string() {
        assert_eq!(EditorTypeEnum::from_string("admin"), EditorTypeEnum::Admin);
        assert_eq!(EditorTypeEnum::from_string("ADMIN"), EditorTypeEnum::Admin);
        assert_eq!(EditorTypeEnum::from_string("member"), EditorTypeEnum::Member);
        assert_eq!(EditorTypeEnum::from_string("MEMBER"), EditorTypeEnum::Member);
        assert_eq!(EditorTypeEnum::from_string("guest"), EditorTypeEnum::Guest);
        assert_eq!(EditorTypeEnum::from_string("GUEST"), EditorTypeEnum::Guest);
        assert_eq!(EditorTypeEnum::from_string("invalid"), EditorTypeEnum::None);
        assert_eq!(EditorTypeEnum::from_string(""), EditorTypeEnum::None);
    }

    #[test]
    fn test_from_comma_str() {
        assert_eq!(
            EditorTypeEnum::from_comma_str("admin,guest"),
            vec![EditorTypeEnum::Admin, EditorTypeEnum::Guest]
        );
        assert_eq!(
            EditorTypeEnum::from_comma_str("Admin, MEMBER "),
            vec![EditorTypeEnum::Admin, EditorTypeEnum::Member]
        );
        assert_eq!(
            EditorTypeEnum::from_comma_str("admin,invalid,guest"),
            vec![EditorTypeEnum::Admin, EditorTypeEnum::Guest]
        );
        assert_eq!(
            EditorTypeEnum::from_comma_str(" none "),
            Vec::<EditorTypeEnum>::new()
        );
        assert_eq!(
            EditorTypeEnum::from_comma_str(""),
            Vec::<EditorTypeEnum>::new()
        );
        assert_eq!(
            EditorTypeEnum::from_comma_str("invalid,none"),
            Vec::<EditorTypeEnum>::new()
        );
    }

    #[test]
    fn test_to_comma_str() {
        assert_eq!(
            EditorTypeEnum::to_comma_str(&vec![
                EditorTypeEnum::Admin,
                EditorTypeEnum::Member,
                EditorTypeEnum::Guest
            ]),
            "admin,member,guest"
        );
        assert_eq!(
            EditorTypeEnum::to_comma_str(&vec![
                EditorTypeEnum::Admin,
                EditorTypeEnum::None,
                EditorTypeEnum::Guest
            ]),
            "admin,guest"
        );
        assert_eq!(EditorTypeEnum::to_comma_str(&vec![]), "");
    }

    #[test]
    fn test_deserialize() {
        use serde_json::json;

        let admin: EditorTypeEnum = serde_json::from_value(json!("admin")).unwrap();
        assert_eq!(admin, EditorTypeEnum::Admin);

        let member: EditorTypeEnum = serde_json::from_value(json!("MEMBER")).unwrap();
        assert_eq!(member, EditorTypeEnum::Member);

        let guest: EditorTypeEnum = serde_json::from_value(json!("Guest")).unwrap();
        assert_eq!(guest, EditorTypeEnum::Guest);

        let none: EditorTypeEnum = serde_json::from_value(json!("invalid")).unwrap();
        assert_eq!(none, EditorTypeEnum::None);

        let empty: EditorTypeEnum = serde_json::from_value(json!("")).unwrap();
        assert_eq!(empty, EditorTypeEnum::None);
    }

    #[test]
    fn test_into_select_option_model() {
        let option: SelectOptionModel = EditorTypeEnum::Admin.into();
        assert_eq!(option.label, "管理员");
        assert_eq!(option.value, SelectValueEnum::Str("admin"));

        let option: SelectOptionModel = EditorTypeEnum::Member.into();
        assert_eq!(option.label, "会员");
        assert_eq!(option.value, SelectValueEnum::Str("member"));

        let option: SelectOptionModel = EditorTypeEnum::Guest.into();
        assert_eq!(option.label, "游客");
        assert_eq!(option.value, SelectValueEnum::Str("guest"));

        let option: SelectOptionModel = EditorTypeEnum::None.into();
        assert_eq!(option.label, "无效值");
        assert_eq!(option.value, SelectValueEnum::Str("none"));
    }
}