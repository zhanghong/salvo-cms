use salvo::prelude::*;

mod address_handler;
mod app_handler;
mod checker_handler;
mod item_handler;
mod kind_handler;
mod morph_handler;

use cms_core::middleware::jwt_verify_access;

pub fn init_router() -> Router {
    Router::new()
        .push(
            Router::with_path("/checker")
                .push(Router::with_path("/health").get(checker_handler::health))
                .push(Router::with_path("/database").get(checker_handler::database)),
        )
        .push(
            Router::with_path("/manage")
                .hoop(jwt_verify_access)
                // App 管理
                .push(Router::with_path("/apps").get(app_handler::manager_paginate))
                .push(Router::with_path("/apps").post(app_handler::manager_create))
                .push(Router::with_path("/apps/query").get(app_handler::manager_query))
                .push(Router::with_path("/apps/form").get(app_handler::manager_form))
                .push(Router::with_path("/apps/redis_store").get(address_handler::redis_store))
                .push(Router::with_path("/apps/redis_load").get(address_handler::redis_load))
                .push(Router::with_path("/apps/{id}/bool").patch(app_handler::update_bool_field))
                .push(Router::with_path("/apps/{id}").get(app_handler::manager_view))
                .push(Router::with_path("/apps/{id}").patch(app_handler::manager_update))
                .push(Router::with_path("/apps/{id}").delete(app_handler::manager_delete))
                // Kind 管理
                .push(Router::with_path("/kinds").get(kind_handler::manager_paginate))
                .push(Router::with_path("/kinds").post(kind_handler::manager_create))
                .push(Router::with_path("/kinds/query").get(kind_handler::manager_query))
                .push(Router::with_path("/kinds/form").get(kind_handler::manager_form))
                .push(Router::with_path("/kinds/{id}/bool").patch(kind_handler::update_bool_field))
                .push(Router::with_path("/kinds/{id}").get(kind_handler::manager_view))
                .push(Router::with_path("/kinds/{id}").patch(kind_handler::manager_update))
                .push(Router::with_path("/kinds/{id}").delete(kind_handler::manager_delete))
                // Item 管理
                .push(Router::with_path("/items").get(item_handler::manager_paginate))
                .push(Router::with_path("/items").post(item_handler::manager_create))
                .push(Router::with_path("/items/query").get(item_handler::manager_query))
                .push(Router::with_path("/items/form").get(item_handler::manager_form))
                .push(Router::with_path("/items/{id}/bool").patch(item_handler::update_bool_field))
                .push(Router::with_path("/items/{id}").get(item_handler::manager_view))
                .push(Router::with_path("/items/{id}").patch(item_handler::manager_update))
                .push(Router::with_path("/items/{id}").delete(item_handler::manager_delete))
                // Morph 管理
                .push(Router::with_path("/morphs/list").get(morph_handler::manager_list)),
        )
}
