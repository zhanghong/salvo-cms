use salvo::cors::Cors;
use salvo::http::Method;
use salvo::prelude::*;

use cms_auth::handler::init_router as auth_router;
use cms_core::config::AppState;
use cms_core::handler::init_router as core_router;
use cms_core::middleware::jwt_authorizor_init;
use cms_mate::handler::init_router as mate_router;
use cms_user::handler::init_router as user_router;

pub fn init_router(state: AppState) -> Router {
    let jwt_auth = jwt_authorizor_init();
    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers("authorization")
        .into_handler();

    Router::new()
        .hoop(cors)
        .hoop(jwt_auth)
        .hoop(affix_state::inject(state))
        .push(Router::with_path("/auth").push(auth_router()))
        .push(Router::with_path("/core").push(core_router()))
        .push(Router::with_path("/mate").push(mate_router()))
        .push(Router::with_path("/user").push(user_router()))
}
