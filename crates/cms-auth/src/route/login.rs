use salvo::oapi::extract::*;
use salvo::prelude::*;
use validator::Validate;

use cms_core::{
    config::AppState,
    domain::{result_ok, AppResult},
    enums::PlatformEnum,
};

use crate::{domain::form::LoginByPasswordForm, service::LoginService};

/// 密码登录
///
/// 管理端密码登录
#[endpoint(
  tags("权鉴模块/管理端/登录"),
  responses(
      (status_code = 200, description = "success response")
  )
)]
pub async fn manager_create(
    depot: &mut Depot,
    json: JsonBody<LoginByPasswordForm>,
) -> AppResult<String> {
    let form = json.into_inner();
    form.validate()?;
    let state = depot.obtain::<AppState>().unwrap();
    let dto = form.into();
    LoginService::store(&PlatformEnum::Manager, &dto, &state.db).await?;
    result_ok("oK".to_string())
}
