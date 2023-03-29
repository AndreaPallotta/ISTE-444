use std::borrow::Borrow;
use std::collections::HashMap;
use axum::Extension;
use axum::extract::Path;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Number, json, Map};
use crate::db::{Database, create_update_query};
use crate::models::Item;
use crate::api::{ApiResponse, generate_error};
use arangors::document::{
    options::{RemoveOptions, ReplaceOptions, UpdateOptions},
    response::DocumentResponse,
};
use arangors::{document::options::InsertOptions, Collection, Connection};

#[derive(Deserialize, Debug, Serialize)]
pub struct GetItemReq {
    id: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AddItemReq {
    name: String,
    description: String,
    price: f64,
    quantity: i64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct UpdateItemReq {
    id: String,
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    quantity: Option<i64>,
}

#[derive(Debug, Serialize)]
struct ItemUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<i64>,
}

pub async fn get_item(Extension(database): Extension<Database>, Path(key): Path<String>) -> (StatusCode, Json<ApiResponse<Item>>) {
    let mut bind_vars: HashMap<&str, Value> = HashMap::new();
    bind_vars.insert("key", key.to_owned().into());

    let items: Vec<Item> = database.arango_db.aql_bind_vars("FOR item IN Item FILTER item._key == @key RETURN item", bind_vars).await.unwrap();

    if items.is_empty() {
        (StatusCode::NOT_FOUND, generate_error("No Item Matches Provided ID"))
    } else {
        let item = &items[0];
        (StatusCode::OK, Json(ApiResponse::Success(item.clone())))
    }
}

pub async fn get_items(Extension(database): Extension<Database>) -> (StatusCode, Json<ApiResponse<Vec<Item>>>) {
    let items: Vec<Item> = database.arango_db.aql_str("FOR item IN Item RETURN item").await.unwrap();

    if items.is_empty() {
        (StatusCode::NOT_FOUND, generate_error("No Item Matches Provided ID"))
    } else {
        (StatusCode::OK, Json(ApiResponse::Success(items)))
    }
}

pub async fn add_item(Extension(database): Extension<Database>, Json(payload): Json<AddItemReq>) -> (StatusCode, Json<ApiResponse<Item>>) {
    let name: String = payload.name;
    let description: String = payload.description;
    let price: f64 = payload.price;
    let quantity: i64 = payload.quantity;

    let mut bind_vars: HashMap<&str, Value> = HashMap::new();
    bind_vars.insert("name", Value::String(name));
    bind_vars.insert("description", Value::String(description));
    bind_vars.insert("price", Value::Number(Number::from_f64(price).unwrap()));
    bind_vars.insert("quantity", Value::Number(Number::from(quantity)));

    let query =
    "
    INSERT {
        name: @name,
        description: @description,
        price: @price,
        quantity: @quantity
    } INTO Item
    RETURN NEW
    ";

    let result = database.arango_db.aql_bind_vars(query, bind_vars).await;

    let item = match result {
        Ok(mut items) => items.pop(),
        Err(_) => None,
    };

    match item {
        Some(i) => (StatusCode::OK, Json(ApiResponse::Success(i))),
        None => (StatusCode::INTERNAL_SERVER_ERROR, generate_error("Error creating item")),
    }
}

pub async fn edit_item(Extension(database): Extension<Database>, Json(payload): Json<UpdateItemReq>) -> (StatusCode, Json<ApiResponse<Value>>) {
    let id = payload.id;
    let name = payload.name;
    let description = payload.description;
    let price = payload.price;
    let quantity = payload.quantity;

    let params = ItemUpdate {
        name,
        description,
        price,
        quantity,
    };

    let patch = json!(&params);

    let collection = database.arango_db.collection("Item").await.unwrap();

    let response = collection.update_document(id.to_owned().as_str(), patch, UpdateOptions::builder().return_new(true).build()).await.unwrap();
    let item: Option<&Value> = response.new_doc();

    if item.is_some() {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::Success(item.unwrap().clone())))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, generate_error("Error creating item"))
    }

}

pub async fn delete_items(Extension(database): Extension<Database>) {

}