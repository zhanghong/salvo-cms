use salvo::prelude::*;
use thiserror::Error;

use crate::domain::vo::ApiResponse;

// 自定义错误类型
#[derive(Error, Debug)]
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
