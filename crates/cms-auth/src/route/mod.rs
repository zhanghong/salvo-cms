use salvo::prelude::*;

mod login;

pub fn init_router() -> Router {
    Router::new().push(
        Router::with_path("manage")
            .push(Router::with_path("/login/password").post(login::manager_create)),
    )
}
