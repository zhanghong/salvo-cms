use salvo::prelude::*;

use cms_core::config::AppState;
use cms_core::route::init_router as core_router;
use cms_user::route::init_router as user_router;

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .hoop(affix_state::inject(state))
        .push(core_router())
        .push(user_router())
}
