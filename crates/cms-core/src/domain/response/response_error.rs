use std::collections::HashMap;
use std::fmt::Debug;

use salvo::oapi::ToSchema;
use serde::Serialize;

// 统一响应结构(失败)
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core/Base/AppResponseError"))]
pub struct ResponseError {
    /// 状态码
    #[salvo(schema(required = true, value_type = u32))]
    code: u32,

    /// 错误信息
    #[salvo(schema(required = true, value_type = String))]
    message: String,

    /// 返回数据
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, value_type = HashMap<String, String>))]
    data: Option<HashMap<String, String>>,
}
