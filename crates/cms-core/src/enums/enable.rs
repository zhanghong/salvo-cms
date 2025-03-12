use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::{SelectOptionItem, SelectValueEnum};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum EnableEnum {
    Yes,
    No,
}

impl EnableEnum {
    pub fn as_value(&self) -> bool {
        match self {
            EnableEnum::Yes => true,
            EnableEnum::No => false,
        }
    }

    pub fn option_value(&self) -> i64 {
        match self {
            EnableEnum::Yes => 1,
            EnableEnum::No => 0,
        }
    }

    pub fn as_title(&self) -> String {
        match self {
            EnableEnum::Yes => String::from("启用"),
            EnableEnum::No => String::from("禁用"),
        }
    }

    pub fn to_option_list() -> Vec<SelectOptionItem> {
        vec![EnableEnum::Yes.into(), EnableEnum::No.into()]
    }
}

impl Into<SelectOptionItem> for EnableEnum {
    fn into(self) -> SelectOptionItem {
        let value = self.option_value();
        SelectOptionItem {
            label: self.as_title(),
            value: SelectValueEnum::Number(value),
            ..Default::default()
        }
    }
}
