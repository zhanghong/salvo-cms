use salvo::prelude::*;
use salvo::{http::header::USER_AGENT, oapi::extract::*};
use validator::Validate;

use cms_core::{
    config::AppState,
    domain::{AppResult, dto::JwtClaimsDTO, response::BaseBooleanResponse, result_ok},
    enums::PlatformEnum,
};

use crate::domain::dto::LoginStoreDTO;
use crate::domain::vo::TokenUpdateVO;
use crate::{
    domain::{
        form::PasswordLoginForm,
        response::{TokenCreateResponse, TokenUpdateResponse},
        vo::TokenCreateVO,
    },
    service::LoginService,
};

/// Login By Password
///
/// Login by name and password
#[endpoint(
    operation_id = "auth_app_manager_login_by_password",
    tags("Auth/Manager/Login"),
    status_codes(200, 500),
    responses(
        (status_code = 200, body = inline(TokenCreateResponse))
    )
)]
pub async fn manager_create(
    depot: &mut Depot,
    req: &mut Request,
    json: JsonBody<PasswordLoginForm>,
) -> AppResult<TokenCreateVO> {
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

/// Refresh AccessToken
///
/// Refresh auth access_token
#[endpoint(
    operation_id = "auth_app_manager_update_token",
    security(["bearer" = ["bearer"]]),
    tags("Auth/Manager/Login"),
    status_codes(200, 500),
    responses(
        (status_code = 200, body = inline(TokenUpdateResponse))
    )
)]
pub async fn manager_update(depot: &mut Depot) -> AppResult<TokenUpdateVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let claims: Option<JwtClaimsDTO> = match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let data = depot.jwt_auth_data::<JwtClaimsDTO>().unwrap();
            let claims = data.claims.clone();
            Some(claims)
        }
        _ => None,
    };
    println!("claims: {:?}", claims); // Add this line to print claims

    let vo = LoginService::update(claims, state).await?;
    result_ok(vo)
}

/// Logout
///
/// Delete auth access_token
#[endpoint(
    operation_id = "auth_app_manager_delete_token",
    security(["bearer" = ["bearer"]]),
    tags("Auth/Manager/Login"),
    responses(
        (status_code = 200, body = inline(BaseBooleanResponse))
    )
)]
pub async fn manager_delete(depot: &mut Depot) -> AppResult<bool> {
    let state = depot.obtain::<AppState>().unwrap();
    let claims: Option<JwtClaimsDTO> = match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let data = depot.jwt_auth_data::<JwtClaimsDTO>().unwrap();
            let claims = data.claims.clone();
            Some(claims)
        }
        _ => None,
    };

    LoginService::delete(claims, state).await?;
    result_ok(true)
}

/// Login By Password
///
/// login by name and password
#[endpoint(
    operation_id = "auth_app_open_login_by_password",
    tags("Auth/Open/Login"),
    status_codes(200, 500),
    responses(
        (status_code = 200, body = inline(TokenCreateResponse))
    )
)]
pub async fn open_create(
    depot: &mut Depot,
    req: &mut Request,
    json: JsonBody<PasswordLoginForm>,
) -> AppResult<TokenCreateVO> {
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

    let token = LoginService::store(&PlatformEnum::Open, &dto, state).await?;
    result_ok(token)
}

/// Refresh AccessToken
///
/// user refresh auth access_token
#[endpoint(
    operation_id = "auth_app_open_update_token",
    security(["bearer" = ["bearer"]]),
    tags("Auth/Open/Login"),
    status_codes(200, 500),
    responses(
        (status_code = 200, body = inline(TokenUpdateResponse))
    )
)]
pub async fn open_update(depot: &mut Depot) -> AppResult<TokenUpdateVO> {
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

/// Logout
///
/// user logout delete auth access_token
#[endpoint(
    operation_id = "auth_app_open_delete_token",
    security(["bearer" = ["bearer"]]),
    tags("Auth/Open/Login"),
    status_codes(200, 500),
    responses(
        (status_code = 200, body = inline(BaseBooleanResponse))
    )
)]
pub async fn open_delete(depot: &mut Depot) -> AppResult<bool> {
    let state = depot.obtain::<AppState>().unwrap();
    let claims: Option<JwtClaimsDTO> = match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let data = depot.jwt_auth_data::<JwtClaimsDTO>().unwrap();
            let claims = data.claims.clone();
            Some(claims)
        }
        _ => None,
    };

    LoginService::delete(claims, state).await?;
    result_ok(true)
}
