use axum::{http::StatusCode, Json};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::models::User;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ErrorResponse {
    pub error_msg: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "result", content = "content")]
pub enum ApiResponse<T> {
    Success(T),
    Error(ErrorResponse),
}

#[derive(Deserialize)]
pub struct LoginParams {
    email: String,
    password: String,
}

#[derive[Deserialize]]
pub struct SignupParams {
    first_name: String;
    last_name: String;
    email: String;
    password: String;
}

pub async fn handle_login(Json(payload): Json<LoginParams>) -> (StatusCode, Json<ApiResponse<User>>) {
    let email: String = payload.email;
    let password: String = payload.password;

    let password_match: &str = "Password.1";
    let email_match: &str = "ap4534@rit.edu";

    if password == password_match && email == email_match {
        let user: User = User {
            user_id: "1234".to_string(),
            first_name: "Andrea".to_string(),
            last_name: "Pallotta".to_string(),
            email: "ap4534@rit.edu".to_string(),
            password: "Password".to_string()
        };
        (StatusCode::OK, Json(ApiResponse::Success(user)))
    } else {
        let error: ErrorResponse = ErrorResponse { error_msg: "Error" };
        (StatusCode::NOT_FOUND, Json(ApiResponse::Error(error)))
    }
}

pub async fn handle_signup(Json(payload): Json<SignupParams>) -> (StatusCode, Json<ApiResponse<User>>) {
    let first_name: String = payload.first_name;
    let last_name: String = payload.last_name;
    let email: String = payload.email;
    let password: String = payload.password;

    let user: User = User {
        user_id: "1234".to_string(),
        first_name: "Andrea".to_string(),
        last_name: "Pallotta".to_string(),
        email: "ap4534@rit.edu".to_string(),
        password: "Password".to_string()
    };
    (StatusCode::OK, Json(ApiResponse::Success(user)))
}