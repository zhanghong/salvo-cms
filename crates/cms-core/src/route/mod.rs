use salvo::prelude::*;

mod checker;

pub fn init_router() -> Router {
    Router::new()
        .push(Router::with_path("/checker/health").get(checker::health))
        .push(Router::with_path("/checker/database").get(checker::database))
}
