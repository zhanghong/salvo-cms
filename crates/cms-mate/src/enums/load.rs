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

/// 懒加载 Model
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ItemLoadEnum {
    None,
    Editor,
    App,
    Kind,
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
            _ => Ok(ItemLoadEnum::None),
        }
    }
}

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
