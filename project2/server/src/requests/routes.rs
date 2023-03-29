use axum::{routing::{get, post, put, delete}, Router, Extension};

use crate::db::Database;

use super::auth;
use super::items;

pub async fn create_routes(database: Database) -> Router {
    Router::new()
        .route("/api/auth/login", post(auth::handle_login))
        .route("/api/auth/signup", post(auth::handle_signup))
        .route("/api/get_item/:id", get(items::get_item))
        .route("/api/get_items", get(items::get_items))
        .route("/api/add_item", post(items::add_item))
        .route("/api/edit_items", put(items::edit_items))
        .route("/api/delete_items", delete(items::delete_items))
        .layer(Extension(database))
}