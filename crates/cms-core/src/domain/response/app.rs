use std::collections::HashMap;
use std::fmt::Debug;

use salvo::http::StatusCode;
use salvo::oapi::{self, EndpointOutRegister, ToSchema};
use salvo::prelude::*;
use serde::Serialize;

// 统一响应结构
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "App Response"))]
pub struct AppResponse<T: Serialize> {
    /// 状态码
    code: u32,

    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, value_type = String))]
    message: Option<String>,

    /// 返回数据
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, value_type = T))]
    data: Option<T>,
}

impl<T: Serialize> AppResponse<T> {
    // 成功响应
    pub fn success(data: T) -> Self {
        Self::new(200, None, Some(data))
    }

    // 错误响应
    pub fn error(code: u32, message: &str) -> Self {
        Self::new(code, Some(message.to_string()), None)
    }

    // 错误响应
    pub fn new(code: u32, message: Option<String>, data: Option<T>) -> Self {
        Self {
            code,
            message,
            data,
        }
    }
}

#[async_trait]
impl<T> Writer for AppResponse<T>
where
    T: Serialize + salvo::prelude::ToSchema + Send + Sync + Debug + 'static,
{
    async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        let json_string = serde_json::to_string(&self).unwrap_or_default();
        depot.insert("res_v", json_string);
        res.render(Json(&self));
    }
}

impl<T> EndpointOutRegister for AppResponse<T>
where
    T: Serialize + salvo::prelude::ToSchema + 'static,
{
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation.responses.insert(
            StatusCode::OK.as_str(),
            oapi::Response::new("success")
                .add_content("application/json", Self::to_schema(components)),
        )
    }
}

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

// 统一响应结构(失败)
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "App Response Error"))]
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
