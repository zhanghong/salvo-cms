use serde::{Deserialize, Serialize};

/// 懒加载 Model
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum KindLoadEnum {
    None,
    Editor,
    App,
    Items,
}

// 实现默认值
impl Default for KindLoadEnum {
    fn default() -> Self {
        Self::None
    }
}

// 实现从字符串和数字的反序列化
impl<'de> Deserialize<'de> for KindLoadEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "editor" => Ok(KindLoadEnum::Editor),
            "app" => Ok(KindLoadEnum::App),
            "items" => Ok(KindLoadEnum::Items),
            _ => Ok(KindLoadEnum::None),
        }
    }
}
