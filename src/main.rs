use salvo::prelude::*;

mod handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    let router = Router::new().get(handler::checker::health);
    println!("{:?}", router);
    Server::new(acceptor).serve(router).await;
}
