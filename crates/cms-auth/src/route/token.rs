use salvo::prelude::*;

use cms_core::domain::{result_ok, AppResult};

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
    result_ok(true)
}
