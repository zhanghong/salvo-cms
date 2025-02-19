use salvo::prelude::*;

use cms_core::domain::{result_ok, AppResult};
use cms_core::service::AuthService;

/// 验证AccessToken
///
/// 管理端验证AccessToken
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
