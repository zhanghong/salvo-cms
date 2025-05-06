use serde::{Deserialize, Serialize};

/// 懒加载 Model
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ItemLoadEnum {
    None,
    Editor,
    App,
    Kind,
    Parent,
}

// 实现默认值
impl Default for ItemLoadEnum {
    fn default() -> Self {
        Self::None
    }
}

// 实现从字符串和数字的反序列化
impl<'de> Deserialize<'de> for ItemLoadEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "editor" => Ok(ItemLoadEnum::Editor),
            "app" => Ok(ItemLoadEnum::App),
            "kind" => Ok(ItemLoadEnum::Kind),
            "parent" => Ok(ItemLoadEnum::Parent),
            _ => Ok(ItemLoadEnum::None),
        }
    }
}
