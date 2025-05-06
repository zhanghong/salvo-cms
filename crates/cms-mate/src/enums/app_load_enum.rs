use serde::{Deserialize, Serialize};

/// 懒加载 Model
#[derive(Debug, Clone, PartialEq, Serialize)]
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
