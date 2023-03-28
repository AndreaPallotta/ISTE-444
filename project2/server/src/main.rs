use axum::{
    routing::get,
    Router, http::StatusCode, Json,
};
use std::net::SocketAddr;

mod models;

use models::User;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app: Router = Router::new()
        .route("/", get(root));

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> (StatusCode, Json<User>) {
    let user: User = User {
        user_id: "1234".to_string(),
        first_name: "Andrea".to_string(),
        last_name: "Pallotta".to_string(),
        email: "ap4534@rit.edu".to_string(),
        password: "Password".to_string()
    };

    (StatusCode::OK, Json(user))
}
