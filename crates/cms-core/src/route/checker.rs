use salvo::prelude::*;

use crate::config::AppState;
use crate::domain::{result_ok, AppResult};

/// 服务状态
///
/// 检查服务健康状态
#[endpoint(
    tags("Core/Checker"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn health() -> AppResult<String> {
    result_ok("oK".to_string())
}

/// 数据库状态
///
///  检查数据库连接
#[endpoint(
    tags("Core/Checker"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn database(depot: &mut Depot) -> AppResult<String> {
    let state = depot.obtain::<AppState>().unwrap();
    let _ = &state.db.ping().await?;
    result_ok("oK".to_string())
}
