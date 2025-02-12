use salvo::prelude::*;

use crate::config::AppState;
use crate::handler;

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .hoop(affix_state::inject(state))
        .get(handler::checker::health)
        .push(Router::with_path("/checker/health").get(handler::checker::health))
        .push(Router::with_path("/checker/database").get(handler::checker::database))
}
