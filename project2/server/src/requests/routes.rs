use axum::{routing::{get, post, put, delete}, Router, Extension, http::HeaderName};
use crate::db::Database;
use crate::requests::{auth, items};
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer, cors::CorsLayer,
};


pub async fn create_routes(database: Database) -> Router {
    Router::new()
        .route("/api/auth/login", post(auth::handle_login))
        .route("/api/auth/signup", post(auth::handle_signup))
        .route("/api/get_item/:id", get(items::get_item))
        .route("/api/get_items", get(items::get_items))
        .route("/api/add_item", post(items::add_item))
        .route("/api/edit_item", put(items::edit_item))
        .route("/api/delete_item", delete(items::delete_item))
        .layer(Extension(database))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        .layer(CorsLayer::permissive())
}