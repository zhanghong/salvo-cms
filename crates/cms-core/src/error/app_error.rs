use salvo::http::{StatusCode, errors::ParseError};
use salvo::oapi::{self, Components, EndpointOutRegister, RefOr, Schema, ToSchema};
use salvo::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

use crate::domain::response::{AppResponse, BaseErrorResponse};

// 自定义错误类型
#[derive(Error, Debug, Serialize, Clone, PartialEq)]
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
        $(impl From<$ty> for AppError {
            fn from(err: $ty) -> Self {
                $variant(err.to_string())
            }
        })+
    };
}

impl_from_error!(
    anyhow::Error => AppError::BadRequest,
    sea_orm::DbErr => AppError::Database,
    redis::RedisError => AppError::Redis,
    lapin::Error => AppError::Queue,
    deadpool_lapin::PoolError => AppError::Queue,
    deadpool_lapin::CreatePoolError => AppError::Queue,
    serde_json::Error => AppError::BadRequest,
    ParseError => AppError::BadRequest
);

// 为 AppError 实现 Into<AppResponse<HashMap<String, String>>>
impl Into<AppResponse<HashMap<String, String>>> for AppError {
    fn into(self) -> AppResponse<HashMap<String, String>> {
        let mut code = 500;
        let mut message = String::from("Internal Server Error");
        let mut data: Option<HashMap<String, String>> = None;

        match self {
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

        AppResponse::new(code, Some(message), data)
    }
}

// 为自定义错误实现 Salvo 的 Writer
#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let cloned_self = self.clone(); // 克隆 self 以保留原始错误信息
        let response: AppResponse<HashMap<String, String>> = self.into();

        error!("Error occurred: {:?}", cloned_self); // 使用克隆的错误信息进行日志记录

        res.render(Json(response));
    }
}

impl ToSchema for AppError {
    fn to_schema(components: &mut Components) -> RefOr<Schema> {
        <AppResponse<HashMap<String, String>> as ToSchema>::to_schema(components)
    }
}

impl EndpointOutRegister for AppError {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        for (code, description) in [
            // (StatusCode::CONTINUE, "Continue"),
            // (StatusCode::SWITCHING_PROTOCOLS, "Switching Protocols"),
            // (StatusCode::PROCESSING, "Processing"),
            // (StatusCode::OK, "OK"),
            // (StatusCode::CREATED, "Created"),
            // (StatusCode::ACCEPTED, "Accepted"),
            // (
            //     StatusCode::NON_AUTHORITATIVE_INFORMATION,
            //     "Non-Authoritative Information",
            // ),
            (StatusCode::NO_CONTENT, "No Content"),
            (StatusCode::RESET_CONTENT, "Reset Content"),
            (StatusCode::PARTIAL_CONTENT, "Partial Content"),
            (StatusCode::MULTI_STATUS, "Multi-Status"),
            (StatusCode::ALREADY_REPORTED, "Already Reported"),
            // (StatusCode::IM_USED, "IM Used"),
            // (StatusCode::MULTIPLE_CHOICES, "Multiple Choices"),
            // (StatusCode::MOVED_PERMANENTLY, "Moved Permanently"),
            // (StatusCode::FOUND, "Found"),
            // (StatusCode::SEE_OTHER, "See Other"),
            // (StatusCode::NOT_MODIFIED, "Not Modified"),
            // (StatusCode::USE_PROXY, "Use Proxy"),
            // (StatusCode::TEMPORARY_REDIRECT, "Temporary Redirect"),
            // (StatusCode::PERMANENT_REDIRECT, "Permanent Redirect"),
            (StatusCode::BAD_REQUEST, "Bad Request"),
            (StatusCode::UNAUTHORIZED, "Unauthorized"),
            (StatusCode::PAYMENT_REQUIRED, "Payment Required"),
            (StatusCode::FORBIDDEN, "Forbidden"),
            (StatusCode::NOT_FOUND, "Not Found"),
            // (StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed"),
            // (StatusCode::NOT_ACCEPTABLE, "Not Acceptable"),
            // (
            //     StatusCode::PROXY_AUTHENTICATION_REQUIRED,
            //     "Proxy Authentication Required",
            // ),
            (StatusCode::REQUEST_TIMEOUT, "Request Timeout"),
            // (StatusCode::CONFLICT, "Conflict"),
            // (StatusCode::GONE, "Gone"),
            // (StatusCode::LENGTH_REQUIRED, "Length Required"),
            // (StatusCode::PRECONDITION_FAILED, "Precondition Failed"),
            // (
            //     StatusCode::REQUEST_ENTITY_TOO_LARGE,
            //     "Request Entity Too Large",
            // ),
            // (StatusCode::REQUEST_URI_TOO_LONG, "Request-URI Too Long"),
            // (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported Media Type"),
            // (
            //     StatusCode::REQUESTED_RANGE_NOT_SATISFIABLE,
            //     "Requested Range Not Satisfiable",
            // ),
            // (StatusCode::EXPECTATION_FAILED, "Expectation Failed"),
            // (StatusCode::IM_A_TEAPOT, "I'm a teapot"),
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            (StatusCode::NOT_IMPLEMENTED, "Not Implemented"),
            (StatusCode::BAD_GATEWAY, "Bad Gateway"),
            (StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable"),
        ] {
            operation.responses.insert(
                code.as_str(),
                oapi::Response::new(description)
                    .add_content("application/json", BaseErrorResponse::to_schema(components)),
            );
        }
    }
}
