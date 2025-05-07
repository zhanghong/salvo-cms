use salvo::prelude::*;

mod checker_handler;
mod login_handler;
mod route_handler;

use cms_core::middleware::{jwt_verify_access, jwt_verify_refresh};

pub fn init_router() -> Router {
    Router::new()
        .push(Router::with_path("/checker/health").get(checker_handler::health))
        .push(Router::with_path("/checker/database").get(checker_handler::database))
        .push(Router::with_path("/manage/login/password").post(login_handler::manager_create))
        .push(Router::with_path("/manage/open/password").post(login_handler::open_create))
        .push(
            Router::with_hoop(jwt_verify_refresh)
                .push(Router::with_path("/manage/login").patch(login_handler::manager_update))
                .push(Router::with_path("/open/login").patch(login_handler::open_update)),
        )
        .push(
            Router::with_path("manage")
                .hoop(jwt_verify_access)
                .push(Router::with_path("/login").delete(login_handler::manager_delete))
                .push(Router::with_path("/routes/list").get(route_handler::manager_list)),
        )
        .push(
            Router::with_path("open")
                .hoop(jwt_verify_access)
                .push(Router::with_path("/login").delete(login_handler::open_delete)),
        )
}
