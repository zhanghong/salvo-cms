use anyhow::Result as AnyResult;
use serde::Serialize;

use crate::error::AppError;

pub mod dto;
pub mod entity;
pub mod form;
pub mod model;
pub mod schemas;
pub mod vo;

mod response;

pub use response::AppResponse;

pub type HandleResult<T> = AnyResult<T, AppError>;
pub type AppResult<T> = AnyResult<AppResponse<T>, AppError>;

pub fn result_ok<T: Serialize>(data: T) -> AppResult<T> {
    Ok(AppResponse::success(data))
}

pub fn handle_ok<T>(data: T) -> HandleResult<T> {
    Ok(data)
}
