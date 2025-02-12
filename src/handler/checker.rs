use salvo::prelude::*;

use crate::domain::vo::{result_ok, ApiResult};

/// Health check
///
/// Handles the health check endpoint.
#[endpoint(status_codes(200, 500))]
pub async fn health() -> ApiResult<String> {
    result_ok("oK".to_string())
}
