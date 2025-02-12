use salvo::prelude::*;

mod domain;
mod error;
mod handler;

use domain::vo::{ApiResponse, ApiResult};

// 使用示例
#[handler]
async fn get_user() -> ApiResult<String> {
    Ok(ApiResponse::success("oK".to_string()))
}

#[handler]
async fn create_user() -> ApiResult<String> {
    Ok(ApiResponse::success("oK".to_string()))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    let router = Router::new().get(handler::checker::health);
    println!("{:?}", router);
    Server::new(acceptor).serve(router).await;
}
