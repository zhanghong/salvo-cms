use std::fmt::Debug;

use salvo::oapi::ToSchema;
use serde::Serialize;

/// 统一响应结构(成功)
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "App Response Success"))]
pub struct ResponseSuccess<T: Serialize> {
    /// 状态码
    code: u32,

    /// 返回数据
    #[salvo(schema(required = true, value_type = T))]
    data: T,
}
