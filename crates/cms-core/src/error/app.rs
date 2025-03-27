use salvo::http::{StatusCode, StatusError};
use salvo::oapi::{self, EndpointOutRegister, ToSchema};
use salvo::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

use crate::domain::AppResponse;

// 自定义错误类型
#[derive(Error, Debug, Serialize, Clone)]
pub enum AppError {
    #[error("Internal server error")]
    Internal,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Redis error: {0}")]
    Redis(String),

    #[error("Queue error: {0}")]
    Queue(String),

    #[error(transparent)]
    Validation(#[from] ValidationErrors),
}

// 宏生成 From 实现，减少重复代码
macro_rules! impl_from_error {
    ($($ty:ty => $variant:path),+) => {
        $(
            impl From<$ty> for AppError {
                fn from(err: $ty) -> Self {
                    $variant(err.to_string())
                }
            }
        )+
    };
}

impl_from_error!(
    anyhow::Error => AppError::BadRequest,
    sea_orm::DbErr => AppError::Database,
    redis::RedisError => AppError::Redis,
    lapin::Error => AppError::Queue,
    deadpool_lapin::PoolError => AppError::Queue,
    deadpool_lapin::CreatePoolError => AppError::Queue
);

// 提取状态码和消息映射逻辑
fn map_error_to_response(error: AppError) -> (u32, String, Option<HashMap<String, String>>) {
    let mut code = 500;
    let mut message = String::from("Internal Server Error");
    let mut data: Option<HashMap<String, String>> = None;

    match error {
        AppError::BadRequest(msg) => {
            code = 400;
            message = msg;
        }
        AppError::Unauthorized => {
            code = 401;
            message = String::from("Unauthorized");
        }
        AppError::NotFound(msg) => {
            code = 404;
            message = msg;
        }
        AppError::Validation(err) => {
            code = 400;
            message = String::from("Validation failed");
            let mut map = HashMap::new();
            for (field, messages) in err.field_errors() {
                if let Some(msg) = messages.first() {
                    map.insert(field.to_string(), msg.to_string());
                }
            }
            data = Some(map);
        }
        _ => {}
    }

    (code, message, data)
}

// 为自定义错误实现 Salvo 的 Writer
#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let cloned_self = self.clone(); // 克隆 self 以保留原始错误信息
        let (code, message, data) = map_error_to_response(self);

        error!("Error occurred: {:?}", cloned_self); // 使用克隆的错误信息进行日志记录

        let response = AppResponse::new(code, Some(message), data);
        res.render(Json(response));
    }
}

// 提取 OpenAPI 响应注册逻辑
const OPENAPI_RESPONSES: [(u16, &str, &str); 3] = [
    (500, "Internal server error", "Internal server error"),
    (404, "Not found", "Not found"),
    (400, "Bad request", "Bad request"),
];

impl EndpointOutRegister for AppError {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        for &(code, description, _) in &OPENAPI_RESPONSES {
            operation.responses.insert(
                StatusCode::from_u16(code).unwrap().as_str(),
                oapi::Response::new(description)
                    .add_content("application/json", StatusError::to_schema(components)),
            );
        }
    }
}
