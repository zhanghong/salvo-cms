use salvo::prelude::*;

use crate::domain::vo::{ApiResponse, ApiResult};

#[handler]
pub async fn health() -> ApiResult<String> {
    Ok(ApiResponse::success("oK".to_string()))
}
