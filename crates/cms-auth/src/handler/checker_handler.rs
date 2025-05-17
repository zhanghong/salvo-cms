use salvo::prelude::*;

use cms_core::config::AppState;
use cms_core::domain::{
    AppResult,
    response::{BaseBooleanResponse, BaseStringResponse},
    result_ok,
};

/// Service Status
///
/// Check service status
#[endpoint(
    operation_id = "auth_service_health_checker",
    tags("Auth/Checker"),
    status_codes(200, 500),
    responses(
        (status_code = 200, body = BaseStringResponse)
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
    tags("Auth/Checker"),
    status_codes(200, 500),
    responses(
        (status_code = 200, body = BaseBooleanResponse)
    )
)]
pub async fn database(depot: &mut Depot) -> AppResult<bool> {
    let state = depot.obtain::<AppState>().unwrap();
    let res = &state.db.ping().await;
    result_ok(res.is_ok())
}
