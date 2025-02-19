use salvo::prelude::*;

use cms_auth::route::init_router as auth_router;
use cms_core::config::AppState;
use cms_core::middleware::jwt_authorizor;
use cms_core::route::init_router as core_router;
use cms_user::route::init_router as user_router;

pub fn init_router(state: AppState) -> Router {
    let jwt_auth = jwt_authorizor();
    Router::new()
        .hoop(affix_state::inject(state))
        .hoop(jwt_auth)
        .push(Router::with_path("/auth").push(auth_router()))
        .push(Router::with_path("/core").push(core_router()))
        .push(Router::with_path("/user").push(user_router()))
}
