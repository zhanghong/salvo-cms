use salvo::prelude::*;

mod checker;

pub fn init_router() -> Router {
    Router::new().push(
        Router::with_path("/checker")
            .push(Router::with_path("/health").get(checker::health))
            .push(Router::with_path("/database").get(checker::database)),
    )
}
