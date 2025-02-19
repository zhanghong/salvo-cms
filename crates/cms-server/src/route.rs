use cms_core::domain::dto::JwtClaimsDTO;
use salvo::jwt_auth::{ConstDecoder, QueryFinder};
use salvo::prelude::*;

use cms_auth::route::init_router as auth_router;
use cms_core::config::AppState;
use cms_core::route::init_router as core_router;
use cms_user::route::init_router as user_router;

const SECRET_KEY: &str = "YOUR SECRET_KEY";

pub fn init_router(state: AppState) -> Router {
    let token = QueryFinder::new("jwt_token");
    let jwt_auth: JwtAuth<JwtClaimsDTO, _> =
        JwtAuth::new(ConstDecoder::from_secret(SECRET_KEY.as_bytes()))
            .finders(vec![
                // Box::new(HeaderFinder::new()),
                Box::new(token),
                // Box::new(CookieFinder::new("jwt_token")),
            ])
            .force_passed(true);

    Router::new()
        .hoop(affix_state::inject(state))
        .hoop(jwt_auth)
        .push(Router::with_path("/auth").push(auth_router()))
        .push(Router::with_path("/core").push(core_router()))
        .push(Router::with_path("/user").push(user_router()))
}
