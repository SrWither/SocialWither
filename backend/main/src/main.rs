pub mod api;
pub mod routes;

use crate::routes::router;
use salvo::prelude::*;
use dotenv;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter("error")
        .with_env_filter("warn")
        .with_env_filter("info")
        .init();

    dotenv::dotenv().ok();

    let acceptor = TcpListener::new("127.0.0.1:5400").bind().await;
    Server::new(acceptor).serve(router()).await;
}
