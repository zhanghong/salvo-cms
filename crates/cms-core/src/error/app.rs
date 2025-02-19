use std::collections::HashMap;

use salvo::http::{StatusCode, StatusError};
use salvo::oapi::{self, EndpointOutRegister, ToSchema};
use salvo::prelude::*;
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

use crate::domain::AppResponse;

// 自定义错误类型
#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error("Internal server error")]
    Internal,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Database error: {0}")]
    Database(String),

    #[error(transparent)]
    Validation(#[from] ValidationErrors),
}

// 将 anyhow::Error 转换为自定义错误
impl From<anyhow::Error> for AppError {
    fn from(_: anyhow::Error) -> Self {
        AppError::Internal
    }
}

// 修改 From 实现
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        println!("Database error: {:?}", err);
        AppError::Database(err.to_string()) // 将 DbErr 转换为字符串
    }
}

// impl From<ValidationErrors> for AppError {
//     fn from(err: ValidationErrors) -> Self {
//         AppError::Validation(err.to_string())
//     }
// }

// 为自定义错误实现 Salvo 的 Writer
#[async_trait]
impl Writer for AppError {
    // 实现Writer trait的write方法
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let mut code = 500;
        let mut message = String::from("Internal Server Error");
        let mut data: Option<HashMap<String, String>> = None;
        println!("AppError: {:?}", self);
        match self {
            AppError::BadRequest(msg) => {
                code = 400;
                message = msg;
            }
            AppError::Unauthorized => {
                code = 401;
                message = String::from("Unauthorized");
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
        let response = AppResponse::new(code, Some(message), data);
        // 将错误信息渲染为Json格式
        res.render(Json(response));
    }
}

impl EndpointOutRegister for AppError {
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
