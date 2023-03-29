use std::collections::HashMap;
use axum::Extension;
use axum::{http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;
use crate::db::Database;
use crate::models::User;
use crate::api::{ApiResponse, generate_error};

#[derive(Deserialize)]
pub struct LoginParams {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct SignupParams {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

pub async fn handle_login(Extension(database): Extension<Database>, Json(payload): Json<LoginParams>) -> (StatusCode, Json<ApiResponse<User>>) {
    let email: String = payload.email;
    let password: String = payload.password;

    let mut bind_vars: HashMap<&str, Value> = HashMap::new();
    bind_vars.insert("email", email.to_owned().into());

    let users: Vec<User> = database.arango_db.aql_bind_vars("FOR user IN User FILTER user.email == @email RETURN user", bind_vars).await.unwrap();

    if users.is_empty() {
        (StatusCode::NOT_FOUND, generate_error("User not found"))
    } else {
        let user = &users[0];

        if password == user.password {
            (StatusCode::OK, Json(ApiResponse::Success(user.clone())))
        } else {
            (StatusCode::BAD_REQUEST, generate_error("Email and/or password do not match"))
        }

    }
}

pub async fn handle_signup(Extension(database): Extension<Database>, Json(payload): Json<SignupParams>) -> (StatusCode, Json<ApiResponse<User>>) {
    let first_name: String = payload.first_name;
    let last_name: String = payload.last_name;
    let email: String = payload.email;
    let password: String = payload.password;

    let query =
    "
    INSERT {
        first_name: @first_name,
        last_name: @last_name,
        email: @email,
        password: @password
    } INTO User
    RETURN NEW
    ";

    let mut bind_vars = HashMap::new();
    bind_vars.insert("first_name", first_name.into());
    bind_vars.insert("last_name", last_name.into());
    bind_vars.insert("email", email.into());
    bind_vars.insert("password", password.into());

    let result = database.arango_db.aql_bind_vars(query, bind_vars).await;

    let user = match result {
        Ok(mut users) => users.pop(),
        Err(_) => None,
    };

    match user {
        Some(u) => (StatusCode::OK, Json(ApiResponse::Success(u))),
        None => (StatusCode::INTERNAL_SERVER_ERROR, generate_error("Error creating user")),
    }
}