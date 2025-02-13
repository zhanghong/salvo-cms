use salvo::prelude::*;

use crate::config::AppState;
use crate::handler;

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .hoop(affix_state::inject(state))
        .get(handler::checker::health)
        .push(Router::with_path("/checker/health").get(handler::checker::health))
        .push(Router::with_path("/checker/database").get(handler::checker::database))
        .push(
            Router::with_path("/user/manage/users")
                .get(handler::user::manager_paginate)
                .post(handler::user::manager_create),
        )
        .push(Router::with_path("/user/manage/logs").get(handler::user::manager_logs))
        .push(
            Router::with_path("/user/open/users")
                .get(handler::user::open_paginate)
                .post(handler::user::open_create),
        )
}
