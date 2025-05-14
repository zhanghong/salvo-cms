use salvo::prelude::*;

use cms_core::domain::{AppResult, result_ok};

/// Async Route List
///
/// Manager route list
#[endpoint(
  operation_id = "auth_route_manager_list",
  tags("Auth/Manager/Route"),
  status_codes(200, 400),
  responses(
      (status_code = 200, description = "success response")
  )
)]
pub async fn manager_list() -> AppResult<Vec<String>> {
    let list: Vec<String> = vec![];
    result_ok(list)
}
