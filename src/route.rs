use salvo::prelude::*;

pub use crate::handler;

pub fn init_router() -> Router {
    Router::new().get(handler::checker::health)
}
