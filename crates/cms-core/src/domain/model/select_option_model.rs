use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::enums::SelectValueEnum;

/// 通用的选择项结构体，用于下拉选择等场景
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SelectOptionModel {
    /// 显示标签
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "商品"))]
    pub label: String,

    /// 选择值
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "1"))]
    pub value: SelectValueEnum,

    /// 是否禁用
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = bool, example = false, default = false))]
    pub disabled: Option<bool>,

    /// 分组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = String, example = "product"))]
    pub group: Option<String>,

    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = String, example = "product"))]
    pub alias: Option<Vec<String>>,

    /// 子选项
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = SelectOptionModel))]
    pub children: Option<Vec<SelectOptionModel>>,
}
