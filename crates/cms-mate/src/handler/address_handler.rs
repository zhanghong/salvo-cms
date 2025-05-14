use salvo::prelude::*;

use cms_core::{
    config::AppState,
    domain::{AppResult, result_ok},
};

use crate::service::AddressService;

/// Redis Store
///
/// redis store demo
#[endpoint(
  operation_id = "mate_redis_store",
  tags("Mate/Manager/Address"),
  status_codes(200, 400),
  responses(
    (status_code = 200, description = "success response")
  )
)]
pub async fn redis_store(depot: &mut Depot) -> AppResult<()> {
    let _state = depot.obtain::<AppState>().unwrap();
    AddressService::redis_store().await?;
    result_ok(())
}

/// Redis Read
///
/// redis read demo
#[endpoint(
    operation_id = "mate_redis_read",
    tags("Mate/Manager/Address"),
    status_codes(200, 400),
    responses(
      (status_code = 200, description = "success response")
    )
  )]
pub async fn redis_load(depot: &mut Depot) -> AppResult<()> {
    //   let state = depot.obtain::<AppState>().unwrap();
    //   let vo: AppMasterVO = AddressService::redis_load(state).await?;
    let _state = depot.obtain::<AppState>().unwrap();
    AddressService::redis_store().await?;
    result_ok(())
}
