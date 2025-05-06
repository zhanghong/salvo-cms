use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::model::SelectOptionModel;

/// Kind 表单选项 VO
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
#[salvo(schema(name = "Mate模块/Kind/Kind表单选项VO"))]
pub struct KindFormOptionVO {
    /// App 选项
    pub apps: Vec<SelectOptionModel>,

    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionModel>>,
}
