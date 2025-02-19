use salvo::jwt_auth::{ConstDecoder, QueryFinder};
use salvo::prelude::*;

use crate::config::JwtConfig;
use crate::domain::dto::JwtClaimsDTO;

pub fn jwt_authorizor() -> JwtAuth<JwtClaimsDTO, ConstDecoder> {
    let cfg = JwtConfig::from_env().expect("Failed to load jwt config");
    let secret_bytes = cfg.access_secret_bytes();
    let token = QueryFinder::new("jwt_token");
    let auth: JwtAuth<JwtClaimsDTO, ConstDecoder> =
        JwtAuth::new(ConstDecoder::from_secret(&secret_bytes))
            .finders(vec![
                // Box::new(HeaderFinder::new()),
                Box::new(token),
                // Box::new(CookieFinder::new("jwt_token")),
            ])
            .force_passed(true);

    auth
}
