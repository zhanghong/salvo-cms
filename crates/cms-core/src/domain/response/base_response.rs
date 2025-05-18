use std::collections::HashMap;
use std::fmt::Debug;

use salvo::oapi::ToSchema;
use serde::Serialize;
use uuid::Uuid;

// 统一响应结构(失败)
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseErrorResponse"))]
pub struct BaseErrorResponse {
    /// 状态码
    #[salvo(schema(required = true, nullable = false, value_type = u32, minimum=300, maximum = 600, example = 500))]
    code: u32,

    /// 错误信息
    #[salvo(schema(
        required = true,
        nullable = false,
        max_length = 200,
        example = "Internal Server Error"
    ))]
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

/// Return string response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseStringResponse"))]
pub struct BaseStringResponse {
    /// 状态码
    #[salvo(schema(required = true, nullable = false, value_type = i32, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// 返回数据
    #[salvo(schema(required = true, nullable = false, max_length = 200, example = "OK"))]
    data: String,
}

/// return boolean response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseBooleanResponse"))]
pub struct BaseBooleanResponse {
    /// 状态码
    #[salvo(schema(required = true, nullable = false, value_type = i32, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// 返回数据
    #[salvo(schema(required = true, nullable = false, example = true))]
    data: bool,
}

/// return uuid response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseUuidResponse"))]
pub struct BaseUuidResponse {
    /// 状态码
    #[salvo(schema(required = true, nullable = false, value_type = i32, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// 返回数据
    #[salvo(schema(required = true, nullable = false, example = true))]
    data: Uuid,
}

/// return u64 response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseU64Response"))]
pub struct BaseU64Response {
    /// 状态码
    #[salvo(schema(required = true, nullable = false, value_type = i32, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// 返回数据
    #[salvo(schema(required = true, nullable = false, example = true))]
    data: u64,
}

/// return u32 response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseU32Response"))]
pub struct BaseU32Response {
    /// 状态码
    #[salvo(schema(required = true, nullable = false, value_type = i32, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// 返回数据
    #[salvo(schema(required = true, nullable = false, example = true))]
    data: u32,
}
