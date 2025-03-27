use salvo::prelude::*;

use cms_core::domain::{AppResult, result_ok};

/// 动态路由列表
///
/// 管理端动态路由列表
#[endpoint(
  tags("权鉴模块/管理端/路由"),
  responses(
      (status_code = 200, description = "success response")
  )
)]
pub async fn manager_list() -> AppResult<Vec<String>> {
    let list: Vec<String> = vec![];
    result_ok(list)
}
