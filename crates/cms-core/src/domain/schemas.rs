use std::fmt::Debug;
use std::collections::HashMap;

use salvo::oapi::ToSchema;
use serde::Serialize;


// 统一响应结构(失败)
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core/Base/Response/Error"))]
pub struct ResponseError {
    /// 状态码
    #[salvo(schema(required = true, value_type = u32, minimum=300, maximum = 600, example = 500))]
    code: u32,

    /// 错误信息
    #[salvo(schema(required = true, max_length = 200, example = "Internal Server Error"))]
    message: String,

    /// 返回数据
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(
    schema(
        required = false, 
        nullable = true, 
        value_type = HashMap<String, String>, 
        example = json!({
            "name": "name is required",
            "title": "title must be between 2 and 10 characters long"
        })
      )
    )]
    data: Option<HashMap<String, String>>,
}

/// 统一响应结构(成功)
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core/Base/Response/SuccessString"))]
pub struct ResponseSuccessString {
    /// 状态码
    #[salvo(schema(required = true, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, value_type = String, max_length = 200, example = "update success"))]
    message: Option<String>,

    /// 返回数据
    #[salvo(schema(required = true, max_length = 200, example = "OK"))]
    data: String,
}
