use salvo::prelude::*;

use cms_core::domain::{result_ok, AppResult};
use cms_core::service::AuthService;

/// 验证 Access Token
///
/// 管理端验证 Access Token
#[endpoint(
  tags("权鉴模块/管理端/Token"),
  responses(
      (status_code = 200, description = "success response")
  )
)]
pub async fn verify_access_token(depot: &Depot) -> AppResult<bool> {
    AuthService::verify_access_token(depot)?;

    result_ok(true)
}

/// 刷新 Access Token
///
/// 管理端刷新 Access Token
#[endpoint(
  tags("权鉴模块/管理端/Token"),
  responses(
      (status_code = 200, description = "success response")
  )
)]
pub async fn update() -> AppResult<bool> {
    AuthService::generate_refresh_token(1, "admin")?;

    result_ok(true)
}

/// 验证 Refresh Token
///
/// 管理端验证 Refresh Token
#[endpoint(
  tags("权鉴模块/管理端/Token"),
  responses(
      (status_code = 200, description = "success response")
  )
)]
pub async fn verify_refresh_token(depot: &Depot) -> AppResult<bool> {
    AuthService::verify_fresh_token(depot)?;

    result_ok(true)
}
