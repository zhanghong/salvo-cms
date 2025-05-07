use serde::{Deserialize, Serialize};

/// 懒加载 Model
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum UserLoadEnum {
    None,
    Editor,
    Detail,
}

// 实现默认值
impl Default for UserLoadEnum {
    fn default() -> Self {
        Self::None
    }
}

// 实现从字符串和数字的反序列化
impl<'de> Deserialize<'de> for UserLoadEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "editor" => Ok(UserLoadEnum::Editor),
            "detail" => Ok(UserLoadEnum::Detail),
            _ => Ok(UserLoadEnum::None),
        }
    }
}
