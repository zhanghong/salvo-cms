use salvo::prelude::*;

mod checker;
mod login;

use cms_core::middleware::{jwt_verify_access, jwt_verify_refresh};

pub fn init_router() -> Router {
    Router::new()
        .push(Router::with_path("/checker/health").get(checker::health))
        .push(Router::with_path("/checker/database").get(checker::database))
        .push(Router::with_path("/manage/login/password").post(login::manager_create))
        .push(Router::with_path("/manage/open/password").post(login::open_create))
        .push(
            Router::with_hoop(jwt_verify_refresh)
                .push(Router::with_path("/manage/login").patch(login::manager_update))
                .push(Router::with_path("/open/login").patch(login::open_update)),
        )
        .push(
            Router::with_path("manage")
                .hoop(jwt_verify_access)
                .push(Router::with_path("/login").delete(login::manager_delete)),
        )
        .push(
            Router::with_path("open")
                .hoop(jwt_verify_access)
                .push(Router::with_path("/login").delete(login::open_delete)),
        )
}
