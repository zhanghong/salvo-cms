use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::SelectOptionItem;

/// App 表单选项 VO
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
#[salvo(schema(name = "Mate模块/App/App表单选项VO"))]
pub struct AppFormOptionVO {
    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = true, nullable = false, value_type = Vec<SelectOptionItem>, example = json!([{"value":true,"label":"启用"},{"value":false,"label":"禁用"}])))]
    pub enables: Option<Vec<SelectOptionItem>>,
}
