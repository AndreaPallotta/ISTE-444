use std::collections::HashMap;
use axum::Extension;
use axum::extract::Path;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::db::Database;
use crate::models::Item;
use crate::api::{ApiResponse, generate_error};

#[derive(Deserialize, Debug, Serialize)]
pub struct GetItem {
    id: String,
}

pub async fn get_item(Extension(database): Extension<Database>, Path(id): Path<String>) -> (StatusCode, Json<ApiResponse<Item>>) {
    println!("{}", id);

    let item: Item = Item {
        _key: "1".to_string(),
        name: "1".to_string(),
        description: "1".to_string(),
        price: 1.4,
        quantity: 2,
    };

    (StatusCode::OK, Json(ApiResponse::Success(item)))
}

pub async fn get_items(Extension(database): Extension<Database>, Json(payload): Json<GetItem>) {

}

pub async fn add_item(Extension(database): Extension<Database>, Json(payload): Json<GetItem>) {

}

pub async fn edit_items(Extension(database): Extension<Database>, Json(payload): Json<GetItem>) {

}

pub async fn delete_items(Extension(database): Extension<Database>, Json(payload): Json<GetItem>) {

}