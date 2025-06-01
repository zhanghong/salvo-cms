use salvo::prelude::*;

mod checker_handler;

pub fn init_router() -> Router {
    Router::new().push(
        Router::with_path("/checker")
            .push(Router::with_path("/health").get(checker_handler::health))
            .push(Router::with_path("/database").get(checker_handler::database)),
    )
}
