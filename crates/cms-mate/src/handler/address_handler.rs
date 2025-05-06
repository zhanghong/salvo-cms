use salvo::prelude::*;

use cms_core::{
    config::AppState,
    domain::{AppResult, result_ok},
};

use crate::service::AddressService;

/// Redis Store
///
/// 管理端 redis store demo
#[endpoint(
  tags("Mate模块/管理端/应用管理"),
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
/// 管理端 redis read demo
#[endpoint(
  tags("Mate模块/管理端/应用管理"),
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
