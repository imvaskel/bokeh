use std::net::SocketAddr;

use axum::{routing::get, Router, extract::Path};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/media/:path", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Path(path): Path<String>) -> String {
    path
}
