use axum::{ routing::post, Router };
use std::net::SocketAddr;
use server::api_requests::handle_login;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app: Router = Router::new()
        .route("/users", post(handle_login));

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
