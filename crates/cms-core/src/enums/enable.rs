// 引入必要的trait
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

// 引入领域模型中的相关类型
use crate::domain::{SelectOptionItem, SelectValueEnum};

// 定义常量字符串
const ENABLE_TITLE: &str = "启用";
const DISABLE_TITLE: &str = "禁用";

// 定义一个枚举类型EnableEnum，用于表示是否启用的状态
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum EnableEnum {
    Yes,
    No,
}

// 实现EnableEnum枚举的成员方法
impl EnableEnum {
    // 将枚举值转换为布尔值
    pub fn as_value(&self) -> bool {
        match self {
            EnableEnum::Yes => true,
            EnableEnum::No => false,
        }
    }

    // 将枚举值转换为整型值
    pub fn option_value(&self) -> i64 {
        match self {
            EnableEnum::Yes => 1,
            EnableEnum::No => 0,
        }
    }

    // 根据枚举值返回对应的字符串描述
    pub fn as_title(&self) -> &'static str {
        match self {
            EnableEnum::Yes => ENABLE_TITLE,
            EnableEnum::No => DISABLE_TITLE,
        }
    }

    // 生成一个包含所有枚举值的选项列表
    pub fn to_option_list() -> Vec<SelectOptionItem> {
        vec![EnableEnum::Yes, EnableEnum::No]
            .into_iter()
            .map(|e| e.into())
            .collect()
    }
}

// 实现EnableEnum到SelectOptionItem的转换
impl Into<SelectOptionItem> for EnableEnum {
    fn into(self) -> SelectOptionItem {
        let value = self.option_value();
        SelectOptionItem {
            label: self.as_title().to_string(), // 转换为 String 类型
            value: SelectValueEnum::Number(value),
            ..Default::default()
        }
    }
}
