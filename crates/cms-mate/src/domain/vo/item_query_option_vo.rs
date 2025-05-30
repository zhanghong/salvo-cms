use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::model::SelectOptionModel;

/// Item 查询表单选项 VO
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
#[salvo(schema(name = "Mate/Item/ItemQueryOptionVO"))]
pub struct ItemQueryOptionVO {
    /// App 选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<SelectOptionModel>>,

    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<SelectOptionModel>>,

    /// 父级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<SelectOptionModel>>,

    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionModel>>,
}
