use salvo::prelude::*;

use crate::config::AppState;
use crate::domain::vo::{result_ok, ApiResult};

/// Check Health
///
/// Handles the app health endpoint.
#[endpoint(status_codes(200, 500))]
pub async fn manager_paginate() -> ApiResult<String> {
    result_ok("oK".to_string())
}

/// Check Database Connection
///
/// Handles the database connection endpoint.
#[endpoint(status_codes(200, 500))]
pub async fn manager_create(depot: &mut Depot) -> ApiResult<String> {
    let state = depot.obtain::<AppState>().unwrap();
    let _ = &state.db.ping().await?;
    result_ok("oK".to_string())
}
