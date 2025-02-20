use salvo::prelude::*;

mod checker;
mod login;
mod token;

// use cms_core::middleware::jwt_authorizor_check;

pub fn init_router() -> Router {
    Router::new()
        .push(
            Router::with_path("/checker")
                .push(Router::with_path("/health").get(checker::health))
                .push(Router::with_path("/database").get(checker::database)),
        )
        .push(
            Router::with_path("manage")
                .push(Router::with_path("/login/password").post(login::manager_create)),
        )
        // .push(Router::new().hoop(jwt_authorizor_check).push(
        //     Router::with_path("manage").push(
        //         Router::with_path("/token/verify_access_token").get(token::verify_access_token),
        //     ),
        // ))
        .push(Router::with_path("/token/update").get(token::update))
}
