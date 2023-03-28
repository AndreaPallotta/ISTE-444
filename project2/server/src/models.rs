use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct User {
    #[serde(rename = "_id")]
    pub user_id: String,

    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Order {
    #[serde(rename = "_id")]
    pub order_id: String,

    pub date: NaiveDateTime,
    pub user_id: String,
    pub item_ids: Vec<String>,
    pub quantity: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Item {
    #[serde(rename = "_id")]
    pub item_id: String,

    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i64,
}