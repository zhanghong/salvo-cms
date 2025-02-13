use salvo::prelude::*;

mod user;

pub fn init_router() -> Router {
    Router::new()
        .push(
            Router::with_path("manage")
                .push(
                    Router::with_path("/users")
                        .get(user::manager_paginate)
                        .post(user::manager_create),
                )
                .push(Router::with_path("/logs").get(user::manager_logs)),
        )
        .push(
            Router::with_path("/open").push(
                Router::with_path("/users")
                    .get(user::open_paginate)
                    .post(user::open_create),
            ),
        )
}
