use salvo::prelude::*;

mod checker_handler;
mod user_handler;

pub fn init_router() -> Router {
    Router::new()
        .push(
            Router::with_path("/checker")
                .push(Router::with_path("/health").get(checker_handler::health))
                .push(Router::with_path("/database").get(checker_handler::database)),
        )
        .push(
            Router::with_path("/manage")
                .push(
                    Router::with_path("/users")
                        .get(user_handler::manager_paginate)
                        .post(user_handler::manager_create),
                )
                .push(Router::with_path("/users/form").get(user_handler::manager_form))
                .push(Router::with_path("/users/unique").post(user_handler::check_field_unique))
                .push(Router::with_path("/users/{id}/bool").post(user_handler::update_bool_field))
                .push(
                    Router::with_path("/users/{id}")
                        .get(user_handler::manager_view)
                        .patch(user_handler::manager_update)
                        .delete(user_handler::manager_delete),
                )
                .push(
                    Router::with_path("/users/{id}/password")
                        .post(user_handler::manager_update_password),
                )
                .push(Router::with_path("/logs").get(user_handler::manager_logs)),
        )
        .push(
            Router::with_path("/open").push(
                Router::with_path("/users")
                    .get(user_handler::open_paginate)
                    .post(user_handler::open_create),
            ),
        )
}
