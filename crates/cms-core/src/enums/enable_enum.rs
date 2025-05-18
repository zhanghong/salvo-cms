// 引入必要的trait
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

// 引入领域模型中的相关类型
use super::SelectValueEnum;
use crate::consts::enum_consts::*;
use crate::domain::model::SelectOptionModel;

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
            EnableEnum::Yes => ENABLE_TRUE_TITLE,
            EnableEnum::No => ENABLE_FALSE_TITLE,
        }
    }

    // 生成一个包含所有枚举值的选项列表
    pub fn to_option_list() -> Vec<SelectOptionModel> {
        vec![EnableEnum::Yes, EnableEnum::No]
            .into_iter()
            .map(|e| e.into())
            .collect()
    }
}

// 转换成 SelectOptionModel
impl Into<SelectOptionModel> for EnableEnum {
    fn into(self) -> SelectOptionModel {
        let value = self.option_value();
        SelectOptionModel {
            label: self.as_title().to_string(), // 转换为 String 类型
            value: SelectValueEnum::Number(value),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试 as_value 方法
    #[test]
    fn test_as_value() {
        assert!(EnableEnum::Yes.as_value());
        assert!(!EnableEnum::No.as_value());
    }

    // 测试 option_value 方法
    #[test]
    fn test_option_value() {
        assert_eq!(EnableEnum::Yes.option_value(), 1);
        assert_eq!(EnableEnum::No.option_value(), 0);
    }

    // 测试 as_title 方法
    #[test]
    fn test_as_title() {
        assert_eq!(EnableEnum::Yes.as_title(), ENABLE_TRUE_TITLE);
        assert_eq!(EnableEnum::No.as_title(), ENABLE_FALSE_TITLE);
    }

    // 测试 to_option_list 方法
    #[test]
    fn test_to_option_list() {
        let options = EnableEnum::to_option_list();
        assert_eq!(options.len(), 2);

        let yes = &options[0];
        assert_eq!(yes.label, ENABLE_TRUE_TITLE);
        if let SelectValueEnum::Number(v) = yes.value {
            assert_eq!(v, 1);
        } else {
            panic!("Expected Number variant for Yes");
        }

        let no = &options[1];
        assert_eq!(no.label, ENABLE_FALSE_TITLE);
        if let SelectValueEnum::Number(v) = no.value {
            assert_eq!(v, 0);
        } else {
            panic!("Expected Number variant for No");
        }
    }

    // 测试 Into<SelectOptionModel> 的转换
    #[test]
    fn test_into_select_option_model() {
        let yes: SelectOptionModel = EnableEnum::Yes.into();
        assert_eq!(yes.label, ENABLE_TRUE_TITLE);
        if let SelectValueEnum::Number(v) = yes.value {
            assert_eq!(v, 1);
        } else {
            panic!("Expected Number variant for Yes");
        }

        let no: SelectOptionModel = EnableEnum::No.into();
        assert_eq!(no.label, ENABLE_FALSE_TITLE);
        if let SelectValueEnum::Number(v) = no.value {
            assert_eq!(v, 0);
        } else {
            panic!("Expected Number variant for No");
        }
    }
}
