use salvo::prelude::*;

mod address;
mod app;
mod checker;
mod item;
mod kind;
mod morph;

use cms_core::middleware::jwt_verify_access;

pub fn init_router() -> Router {
    Router::new()
        .push(
            Router::with_path("/checker")
                .push(Router::with_path("/health").get(checker::health))
                .push(Router::with_path("/database").get(checker::database)),
        )
        .push(
            Router::with_path("/manage")
                .hoop(jwt_verify_access)
                // App 管理
                .push(Router::with_path("/apps").get(app::manager_paginate))
                .push(Router::with_path("/apps").post(app::manager_create))
                .push(Router::with_path("/apps/query").get(app::manager_query))
                .push(Router::with_path("/apps/form").get(app::manager_form))
                .push(Router::with_path("/apps/redis_store").get(address::redis_store))
                .push(Router::with_path("/apps/redis_load").get(address::redis_load))
                .push(Router::with_path("/apps/{id}/bool").patch(app::update_bool_field))
                .push(Router::with_path("/apps/{id}").get(app::manager_view))
                .push(Router::with_path("/apps/{id}").patch(app::manager_update))
                .push(Router::with_path("/apps/{id}").delete(app::manager_delete))
                // Kind 管理
                .push(Router::with_path("/kinds").get(kind::manager_paginate))
                .push(Router::with_path("/kinds").post(kind::manager_create))
                .push(Router::with_path("/kinds/query").get(kind::manager_query))
                .push(Router::with_path("/kinds/form").get(kind::manager_form))
                .push(Router::with_path("/kinds/{id}/bool").patch(kind::update_bool_field))
                .push(Router::with_path("/kinds/{id}").get(kind::manager_view))
                .push(Router::with_path("/kinds/{id}").patch(kind::manager_update))
                .push(Router::with_path("/kinds/{id}").delete(kind::manager_delete))
                // Item 管理
                .push(Router::with_path("/items").get(item::manager_paginate))
                .push(Router::with_path("/items").post(item::manager_create))
                .push(Router::with_path("/items/query").get(item::manager_query))
                .push(Router::with_path("/items/form").get(item::manager_form))
                .push(Router::with_path("/items/{id}/bool").patch(item::update_bool_field))
                .push(Router::with_path("/items/{id}").get(item::manager_view))
                .push(Router::with_path("/items/{id}").patch(item::manager_update))
                .push(Router::with_path("/items/{id}").delete(item::manager_delete))
                // Morph 管理
                .push(Router::with_path("/morphs/list").get(morph::manager_list)),
        )
}
