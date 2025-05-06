use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::SelectOptionItem;

/// Item 表单选项 VO
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
#[salvo(schema(name = "Mate模块/Item/Item表单选项VO"))]
pub struct ItemFormOptionVO {
    /// App 选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<SelectOptionItem>>,

    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<SelectOptionItem>>,

    /// 父级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<SelectOptionItem>>,

    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionItem>>,
}
