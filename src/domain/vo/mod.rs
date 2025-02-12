use anyhow::Result as AnyResult;

use crate::error::ApiError;

mod app;

pub use app::ApiResponse;

pub type ApiResult<T> = AnyResult<ApiResponse<T>, ApiError>;
