use salvo::prelude::*;

use crate::config::AppState;
use crate::domain::{AppResult, result_ok, schemas::ResponseSuccessString};

/// Service Status
///
/// Check service status
#[endpoint(
    operation_id = "auth_service_health_checker",
    tags("Core/Checker"),
    status_codes(200, 500),
    responses(
        (status_code = 200, body = ResponseSuccessString)
    )
)]
pub async fn health() -> AppResult<String> {
    result_ok("OK".to_string())
}

/// Database Status
///
///  Check database status
#[endpoint(
    operation_id = "auth_database_health_checker",
    tags("Core/Checker"),
    status_codes(200, 500),
    responses(
        (status_code = 200, body = ResponseSuccessString)
    )
)]
pub async fn database(depot: &mut Depot) -> AppResult<String> {
    let state = depot.obtain::<AppState>().unwrap();
    let _ = &state.db.ping().await?;
    result_ok("OK".to_string())
}
