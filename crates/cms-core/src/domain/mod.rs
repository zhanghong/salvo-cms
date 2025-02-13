use anyhow::Result as AnyResult;
use serde::Serialize;

use crate::error::AppError;

mod response;
pub use response::AppResponse;

pub type AppResult<T> = AnyResult<AppResponse<T>, AppError>;

pub fn result_ok<T: Serialize>(data: T) -> AppResult<T> {
    Ok(AppResponse::success(data))
}
