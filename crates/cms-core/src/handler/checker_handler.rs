use salvo::prelude::*;

use crate::config::AppState;
use crate::domain::{
    AppResult,
    response::{BaseBooleanResponse, BaseStringResponse},
    result_ok,
};

/// Service Status
///
/// Check service status
#[endpoint(
    operation_id = "auth_service_health_checker",
    tags("Core/Checker"),
    status_codes(200, 500),
    responses(
        (status_code = 200, body = inline(BaseStringResponse))
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
        (status_code = 200, body = inline(BaseBooleanResponse))
    )
)]
pub async fn database(depot: &mut Depot) -> AppResult<bool> {
    let state = depot.obtain::<AppState>().unwrap();
    let res = &state.db.ping().await;
    result_ok(res.is_ok())
}


#[cfg(test)]
mod tests {
    use salvo::prelude::*;
    use salvo::test::TestClient;

    use crate::handler;

    #[tokio::test]
    async fn test_service_health() {
        let service = Service::new(handler::init_router());

        let response = TestClient::get(format!("http://127.0.0.1:5800/checker/health"))
            .send(&service)
            .await;
        assert_eq!(response.status_code.unwrap(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_database_health() {
        let service = Service::new(handler::init_router());

        let response = TestClient::get(format!("http://127.0.0.1:5800/checker/database"))
            .send(&service)
            .await;
        assert_eq!(response.status_code.unwrap(), StatusCode::OK);
    }
}