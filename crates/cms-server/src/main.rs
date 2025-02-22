use dotenvy::dotenv;
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::oapi::OpenApi;
use salvo::prelude::*;

use cms_core::config::{AppState, WebConfig};

mod route;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let web_config = WebConfig::from_env().expect("Failed to load web config");

    tracing_subscriber::fmt()
        .with_max_level(web_config.tracing_level())
        .with_test_writer()
        .init();

    let state = AppState::init().await;

    let addr = web_config.address();
    println!(
        "🚀 {} service successfully started on http://{}",
        &web_config.app_name(),
        &addr
    );
    tracing::info!(addr, "Server is running");

    let router = route::init_router(state);
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

    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers("authorization")
        .into_handler();

    let service = Service::new(router).hoop(cors);

    let acceptor = TcpListener::new(&addr).bind().await;
    Server::new(acceptor).serve(service).await;
}
