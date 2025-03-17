use salvo::oapi::OpenApi;
use salvo::prelude::*;

use crate::route;
use cms_core::config::{AppState, WebConfig};

pub async fn start() {
  let web_config = WebConfig::from_env().expect("Failed to load web config");

    tracing_subscriber::fmt()
        .with_max_level(web_config.tracing_level())
        .with_test_writer()
        .init();

    let state = AppState::init().await;

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
        .push(Router::with_path("/mate").push(route::init_router()));

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