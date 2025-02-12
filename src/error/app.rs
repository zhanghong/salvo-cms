use salvo::http::{StatusCode, StatusError};
use salvo::oapi::{self, EndpointOutRegister, ToSchema};
use salvo::prelude::*;
use serde::Serialize;
use thiserror::Error;

use crate::domain::vo::ApiResponse;

// 自定义错误类型
#[derive(Error, Debug, Serialize)]
pub enum ApiError {
    #[error("Internal server error")]
    Internal,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Custom error: {0} ({1})")]
    Custom(String, u32),
}

impl ApiError {
    fn status_code(&self) -> u32 {
        match self {
            ApiError::BadRequest(_) => 400,
            ApiError::Unauthorized => 401,
            ApiError::Custom(_, code) => *code,
            _ => 500,
        }
    }
}

// 将 anyhow::Error 转换为自定义错误
impl From<anyhow::Error> for ApiError {
    fn from(_: anyhow::Error) -> Self {
        ApiError::Internal
    }
}

// 为自定义错误实现 Salvo 的 Writer
#[async_trait]
impl Writer for ApiError {
    // 实现Writer trait的write方法
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        // 将错误信息渲染为Json格式
        res.render(Json(ApiResponse::<()>::error(
            self.status_code(),
            &self.to_string(),
        )));
    }
}

impl EndpointOutRegister for ApiError {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation.responses.insert(
            StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            oapi::Response::new("Internal server error")
                .add_content("application/json", StatusError::to_schema(components)),
        );
        operation.responses.insert(
            StatusCode::NOT_FOUND.as_str(),
            oapi::Response::new("Not found")
                .add_content("application/json", StatusError::to_schema(components)),
        );
        operation.responses.insert(
            StatusCode::BAD_REQUEST.as_str(),
            oapi::Response::new("Bad request")
                .add_content("application/json", StatusError::to_schema(components)),
        );
    }
}
