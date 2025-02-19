use dotenvy::dotenv;
use salvo::oapi::OpenApi;
use salvo::prelude::*;

use cms_core::config::{AppState, DbConfig, JwtConfig, WebConfig};

mod domain;
mod route;
mod service;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let web_config = WebConfig::from_env().expect("Failed to load web config");
    let jwt_config = JwtConfig::from_env().expect("Failed to load jwt config");
    let db_config = DbConfig::from_env().expect("Failed to load db config");

    tracing_subscriber::fmt()
        .with_max_level(web_config.tracing_level())
        .with_test_writer()
        .init();

    let db_result = db_config.build_connection().await;
    if db_result.is_err() {
        panic!("Failed to connect to database");
    }

    let state = AppState {
        db: db_result.unwrap().clone(),
        jwt: jwt_config.clone(),
    };

    let addr = web_config.address();
    let acceptor = TcpListener::new(&addr).bind().await;

    println!(
        "🚀 {} service successfully started on http://{}",
        &web_config.app_name(),
        &addr
    );
    tracing::info!(addr, "Server is running");

    let router = Router::new()
        .hoop(affix_state::inject(state))
        .push(Router::with_path("/auth").push(route::init_router()));

    let doc = OpenApi::new(
        web_config.app_name().as_str(),
        web_config.app_version().as_str(),
    )
    .merge_router(&router);

    let api_title = "API Docs";
    let openapi_url = "/api-doc/openapi.json";
    let router = router
        .push(doc.into_router(openapi_url))
        // .push(
        //     SwaggerUi::new(openapi_url)
        //         .title(api_title)
        //         .into_router("swagger-ui"),
        // )
        // .push(
        //     Scalar::new(openapi_url)
        //         .title(api_title)
        //         .into_router("scalar"),
        // )
        // .push(
        //     ReDoc::new(openapi_url)
        //         .title(api_title)
        //         .into_router("redoc"),
        // )
        .push(
            RapiDoc::new(openapi_url)
                .title(api_title)
                .into_router("rapi-doc"),
        );

    Server::new(acceptor).serve(router).await;
}
