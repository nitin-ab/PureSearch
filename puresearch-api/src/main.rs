use puresearch_api::create_app;
use axum::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = create_app();
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

