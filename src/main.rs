use dotenvy::dotenv;
use salvo::oapi::OpenApi;
use salvo::prelude::*;

mod config;
mod domain;
mod error;
pub mod handler;
mod route;

use config::WebConfig;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let web_config = WebConfig::from_env().expect("Failed to load web config");

    tracing_subscriber::fmt()
        .with_max_level(web_config.tracing_level())
        .with_test_writer()
        .init();

    let addr = web_config.address();
    let acceptor = TcpListener::new(&addr).bind().await;

    println!(
        "🚀 {} service successfully started on http://{}",
        &web_config.app_name(),
        &addr
    );
    tracing::info!(addr, "Server is running");

    let router = route::init_router();
    let doc = OpenApi::new(
        web_config.app_name().as_str(),
        web_config.app_version().as_str(),
    )
    .merge_router(&router);

    let openapi_url = "/api-doc/openapi.json";
    let swagger_url = "swagger-ui";
    let router = router
        .push(doc.into_router(openapi_url))
        .push(SwaggerUi::new(openapi_url).into_router(swagger_url));

    Server::new(acceptor).serve(router).await;
}
