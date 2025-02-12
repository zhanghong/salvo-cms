use std::{sync::Arc, time::Duration};

use dotenvy::dotenv;
use salvo::oapi::OpenApi;
use salvo::prelude::*;
use sea_orm::{ConnectOptions, Database};

mod config;
mod domain;
pub mod enums;
mod error;
pub mod handler;
mod route;
pub mod utils;

use config::{AppState, DbConfig, WebConfig};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let web_config = WebConfig::from_env().expect("Failed to load web config");
    let db_config = DbConfig::from_env().expect("Failed to load db config");

    tracing_subscriber::fmt()
        .with_max_level(web_config.tracing_level())
        .with_test_writer()
        .init();

    let mut opt = ConnectOptions::new(db_config.url());
    opt.max_connections(db_config.max_connections.unwrap_or(10))
        .min_connections(db_config.min_connections.unwrap_or(10))
        .connect_timeout(Duration::from_secs(
            db_config.connect_timeout.unwrap_or(10) as u64
        ))
        .acquire_timeout(Duration::from_secs(
            db_config.acquire_timeout.unwrap_or(10) as u64
        ))
        .idle_timeout(Duration::from_secs(
            db_config.idle_timeout.unwrap_or(10) as u64
        ))
        .max_lifetime(Duration::from_secs(
            db_config.max_lifetime.unwrap_or(10) as u64
        ))
        .sqlx_logging(db_config.sqlx_logging.clone().unwrap_or(true));
    let db = Database::connect(opt).await.unwrap();
    let state = AppState { db };

    let addr = web_config.address();
    let acceptor = TcpListener::new(&addr).bind().await;

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

    let openapi_url = "/api-doc/openapi.json";
    let swagger_url = "swagger-ui";
    let router = router
        .push(doc.into_router(openapi_url))
        .push(SwaggerUi::new(openapi_url).into_router(swagger_url));

    Server::new(acceptor).serve(router).await;
}
