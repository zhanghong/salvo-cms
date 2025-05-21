use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use tracing::warn;

use super::SelectValueEnum;
use crate::consts::enum_consts::*;
use crate::domain::model::SelectOptionModel;

// 会员类型
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Enum::EditorTypeEnum"))]
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
            EditorTypeEnum::Admin => EDITOR_TYPE_ADMIN_VALUE,
            EditorTypeEnum::Member => EDITOR_TYPE_MEMBER_VALUE,
            EditorTypeEnum::Guest => EDITOR_TYPE_GUEST_VALUE,
            EditorTypeEnum::None => EDITOR_TYPE_NONE_VALUE,
        }
    }

    // 将枚举值转换为字符串值
    pub fn string_value(&self) -> String {
        self.as_value().to_string()
    }

    // 将枚举值转换为标题字符串
    pub fn as_title(&self) -> &'static str {
        match self {
            EditorTypeEnum::Admin => EDITOR_TYPE_ADMIN_TITLE,
            EditorTypeEnum::Member => EDITOR_TYPE_MEMBER_TITLE,
            EditorTypeEnum::Guest => EDITOR_TYPE_GUEST_TITLE,
            EditorTypeEnum::None => EDITOR_TYPE_NONE_TITLE,
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
            EDITOR_TYPE_ADMIN_VALUE => EditorTypeEnum::Admin,
            EDITOR_TYPE_MEMBER_VALUE => EditorTypeEnum::Member,
            EDITOR_TYPE_GUEST_VALUE => EditorTypeEnum::Guest,
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
                    EDITOR_TYPE_ADMIN_VALUE => EditorTypeEnum::Admin,
                    EDITOR_TYPE_MEMBER_VALUE => EditorTypeEnum::Member,
                    EDITOR_TYPE_GUEST_VALUE => EditorTypeEnum::Guest,
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
            EDITOR_TYPE_ADMIN_VALUE => Ok(EditorTypeEnum::Admin),
            EDITOR_TYPE_MEMBER_VALUE => Ok(EditorTypeEnum::Member),
            EDITOR_TYPE_GUEST_VALUE => Ok(EditorTypeEnum::Guest),
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
        assert_eq!(EditorTypeEnum::Admin.as_value(), EDITOR_TYPE_ADMIN_VALUE);
        assert_eq!(EditorTypeEnum::Member.as_value(), EDITOR_TYPE_MEMBER_VALUE);
        assert_eq!(EditorTypeEnum::Guest.as_value(), EDITOR_TYPE_GUEST_VALUE);
        assert_eq!(EditorTypeEnum::None.as_value(), EDITOR_TYPE_NONE_VALUE);
    }

    #[test]
    fn test_string_value() {
        assert_eq!(
            EditorTypeEnum::Admin.string_value(),
            EDITOR_TYPE_ADMIN_VALUE.to_string()
        );
        assert_eq!(
            EditorTypeEnum::Member.string_value(),
            EDITOR_TYPE_MEMBER_VALUE.to_string()
        );
        assert_eq!(
            EditorTypeEnum::Guest.string_value(),
            EDITOR_TYPE_GUEST_VALUE.to_string()
        );
        assert_eq!(
            EditorTypeEnum::None.string_value(),
            EDITOR_TYPE_NONE_VALUE.to_string()
        );
    }

    #[test]
    fn test_as_title() {
        assert_eq!(EditorTypeEnum::Admin.as_title(), EDITOR_TYPE_ADMIN_TITLE);
        assert_eq!(EditorTypeEnum::Member.as_title(), EDITOR_TYPE_MEMBER_TITLE);
        assert_eq!(EditorTypeEnum::Guest.as_title(), EDITOR_TYPE_GUEST_TITLE);
        assert_eq!(EditorTypeEnum::None.as_title(), EDITOR_TYPE_NONE_TITLE);
    }

    #[test]
    fn test_to_option_list() {
        let options = EditorTypeEnum::to_option_list();
        assert_eq!(options.len(), 3);
        assert_eq!(options[0].label, EDITOR_TYPE_ADMIN_TITLE);
        assert_eq!(
            options[0].value,
            SelectValueEnum::Str(EDITOR_TYPE_ADMIN_VALUE)
        );
        assert_eq!(options[1].label, EDITOR_TYPE_MEMBER_TITLE);
        assert_eq!(
            options[1].value,
            SelectValueEnum::Str(EDITOR_TYPE_MEMBER_VALUE)
        );
        assert_eq!(options[2].label, EDITOR_TYPE_GUEST_TITLE);
        assert_eq!(
            options[2].value,
            SelectValueEnum::Str(EDITOR_TYPE_GUEST_VALUE)
        );
    }

    #[test]
    fn test_from_string() {
        assert_eq!(EditorTypeEnum::from_string("admin"), EditorTypeEnum::Admin);
        assert_eq!(EditorTypeEnum::from_string("ADMIN"), EditorTypeEnum::Admin);
        assert_eq!(
            EditorTypeEnum::from_string("member"),
            EditorTypeEnum::Member
        );
        assert_eq!(
            EditorTypeEnum::from_string("MEMBER"),
            EditorTypeEnum::Member
        );
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
        let string = format!(
            "{},{},{}",
            EDITOR_TYPE_ADMIN_VALUE, EDITOR_TYPE_MEMBER_VALUE, EDITOR_TYPE_GUEST_VALUE
        );
        assert_eq!(
            EditorTypeEnum::to_comma_str(&vec![
                EditorTypeEnum::Admin,
                EditorTypeEnum::Member,
                EditorTypeEnum::Guest
            ]),
            string
        );

        let string = format!("{},{}", EDITOR_TYPE_ADMIN_VALUE, EDITOR_TYPE_GUEST_VALUE);
        assert_eq!(
            EditorTypeEnum::to_comma_str(&vec![
                EditorTypeEnum::Admin,
                EditorTypeEnum::None,
                EditorTypeEnum::Guest
            ]),
            string
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
        assert_eq!(option.label, EDITOR_TYPE_ADMIN_TITLE);
        assert_eq!(option.value, SelectValueEnum::Str(EDITOR_TYPE_ADMIN_VALUE));

        let option: SelectOptionModel = EditorTypeEnum::Member.into();
        assert_eq!(option.label, EDITOR_TYPE_MEMBER_TITLE);
        assert_eq!(option.value, SelectValueEnum::Str(EDITOR_TYPE_MEMBER_VALUE));

        let option: SelectOptionModel = EditorTypeEnum::Guest.into();
        assert_eq!(option.label, EDITOR_TYPE_GUEST_TITLE);
        assert_eq!(option.value, SelectValueEnum::Str(EDITOR_TYPE_GUEST_VALUE));

        let option: SelectOptionModel = EditorTypeEnum::None.into();
        assert_eq!(option.label, EDITOR_TYPE_NONE_TITLE);
        assert_eq!(option.value, SelectValueEnum::Str(EDITOR_TYPE_NONE_VALUE));
    }
}
