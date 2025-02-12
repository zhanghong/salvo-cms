use salvo::prelude::*;

use crate::domain::vo::{result_ok, ApiResponse};

/// Health check
///
/// Handles the health check endpoint.
#[endpoint]
pub async fn health() -> String {
    "data".to_string()
}
