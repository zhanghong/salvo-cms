use salvo::prelude::*;

mod checker;
mod login;
mod token;

pub fn init_router() -> Router {
    Router::new()
        .push(
            Router::with_path("/checker")
                .push(Router::with_path("/health").get(checker::health))
                .push(Router::with_path("/database").get(checker::database)),
        )
        .push(
            Router::with_path("manage")
                .push(Router::with_path("/login/password").post(login::manager_create))
                .push(
                    Router::with_path("/token/verify_access_token").get(token::verify_access_token),
                ),
        )
}
