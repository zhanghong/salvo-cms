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
                .push(Router::with_path("users/form").get(user::manager_form))
                .push(Router::with_path("/users/unique").post(user::check_field_unique))
                .push(Router::with_path("/users/{id}/bool").post(user::update_bool_field))
                .push(
                    Router::with_path("/users/{id}")
                        .get(user::manager_view)
                        .patch(user::manager_update),
                )
                .push(Router::with_path("/users/{id}/password").post(user::manager_update_password))
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
