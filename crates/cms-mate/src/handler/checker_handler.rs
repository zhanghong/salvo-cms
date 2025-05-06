use salvo::prelude::*;

use cms_core::config::AppState;
use cms_core::domain::{AppResult, result_ok};

/// 服务状态
///
/// 检查服务健康状态
#[endpoint(
    tags("Mate模块/Checker"),
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
    tags("Mate模块/Checker"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn database(depot: &mut Depot) -> AppResult<String> {
    let state = depot.obtain::<AppState>().unwrap();
    let _ = &state.db.ping().await?;
    result_ok("oK".to_string())
}
