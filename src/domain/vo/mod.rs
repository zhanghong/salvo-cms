use anyhow::Result as AnyResult;
use serde::Serialize;

use crate::error::ApiError;

mod app;

pub use app::ApiResponse;

pub type ApiResult<T> = AnyResult<ApiResponse<T>, ApiError>;

pub fn result_ok<T: Serialize>(data: T) -> ApiResult<T> {
    Ok(ApiResponse::success(data))
}
