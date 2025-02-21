use salvo::prelude::*;
use salvo::{http::header::USER_AGENT, oapi::extract::*};
use validator::Validate;

use cms_core::{
    config::AppState,
    domain::{dto::JwtClaimsDTO, result_ok, AppResult},
    enums::PlatformEnum,
};

use crate::domain::dto::LoginStoreDTO;
use crate::domain::vo::LoginTokenUpdateVO;
use crate::{
    domain::{form::LoginByPasswordForm, vo::LoginTokenCreateVO},
    service::LoginService,
};

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
    req: &mut Request,
    json: JsonBody<LoginByPasswordForm>,
) -> AppResult<LoginTokenCreateVO> {
    let form = json.into_inner();
    form.validate()?;
    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: LoginStoreDTO = form.into();

    // 获取 User-Agent
    dto.user_agent = req
        .headers()
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    // 获取客户端 IP（支持代理场景）
    dto.client_ip = req.remote_addr().as_ipv4().unwrap().ip().to_string();

    let token = LoginService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(token)
}

/// 刷新 AccessToken
///
/// 管理端刷新 AccessToken
#[endpoint(
    tags("权鉴模块/管理端/登录"),
    responses(
        (status_code = 200, description = "success response")
    )
  )]
pub async fn manager_update(depot: &mut Depot) -> AppResult<LoginTokenUpdateVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let claims: Option<JwtClaimsDTO> = match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let data = depot.jwt_auth_data::<JwtClaimsDTO>().unwrap();
            let claims = data.claims.clone();
            Some(claims)
        }
        _ => None,
    };

    let vo = LoginService::update(claims, state).await?;
    result_ok(vo)
}
