use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::prelude::*;

use crate::config::JwtConfig;
use crate::domain::dto::JwtClaimsDTO;
use crate::domain::{handle_ok, HandleResult};
use crate::service::JwtService;

pub fn jwt_authorizor_init() -> JwtAuth<JwtClaimsDTO, ConstDecoder> {
    let cfg = JwtConfig::from_env().expect("Failed to load jwt config");
    let secret_bytes = cfg.secret_bytes();
    let auth: JwtAuth<JwtClaimsDTO, ConstDecoder> =
        JwtAuth::new(ConstDecoder::from_secret(&secret_bytes))
            .finders(vec![Box::new(HeaderFinder::new())])
            .force_passed(true);

    auth
}

#[handler]
pub fn jwt_verify_access(depot: &mut Depot) -> HandleResult<()> {
    JwtService::verify_access_token(depot)?;
    handle_ok(())
}

#[handler]
pub fn jwt_verify_refresh(depot: &mut Depot) -> HandleResult<()> {
    JwtService::verify_refresh_token(depot)?;
    handle_ok(())
}
