use serde::{Deserialize, Serialize};

/// 懒加载 Model
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum MorphLoadEnum {
    None,
    Editor,
    App,
    Kind,
    Item,
}

// 实现默认值
impl Default for MorphLoadEnum {
    fn default() -> Self {
        Self::None
    }
}

// 实现从字符串和数字的反序列化
impl<'de> Deserialize<'de> for MorphLoadEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "editor" => Ok(MorphLoadEnum::Editor),
            "app" => Ok(MorphLoadEnum::App),
            "kind" => Ok(MorphLoadEnum::Kind),
            "item" => Ok(MorphLoadEnum::Item),
            _ => Ok(MorphLoadEnum::None),
        }
    }
}
