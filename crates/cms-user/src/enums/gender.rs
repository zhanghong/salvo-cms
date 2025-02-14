use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

// 性别枚举
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub enum GenderEnum {
    None,    // 无效值
    Unknown, // 0
    Male,    // 1
    Female,  // 2
}

// 实现默认值
impl Default for GenderEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl GenderEnum {
    pub fn as_value(&self) -> i16 {
        match self {
            GenderEnum::Unknown => 0,
            GenderEnum::Male => 1,
            GenderEnum::Female => 2,
            _ => -1,
        }
    }

    pub fn as_title(&self) -> String {
        let str = match self {
            GenderEnum::Unknown => "保密",
            GenderEnum::Male => "男",
            GenderEnum::Female => "女",
            _ => "异常",
        };
        str.to_string()
    }
}

// 自定义反序列化实现
#[derive(Deserialize)]
#[serde(untagged)]
enum GenderValue {
    Str(String),
    Num(i32),
}

// 实现从字符串和数字的反序列化
impl<'de> Deserialize<'de> for GenderEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = GenderValue::deserialize(deserializer)?;

        Ok(match value {
            GenderValue::Str(s) => match s.as_str() {
                "0" => GenderEnum::Unknown,
                "1" => GenderEnum::Male,
                "2" => GenderEnum::Female,
                _ => GenderEnum::None,
            },
            GenderValue::Num(n) => match n {
                0 => GenderEnum::Unknown,
                1 => GenderEnum::Male,
                2 => GenderEnum::Female,
                _ => GenderEnum::None,
            },
        })
    }
}
