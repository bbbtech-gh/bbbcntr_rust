use serde::{Deserialize, Serialize};
// use chrono::DateTime;
use sqlx::{types::Json};


#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Client {
    pub client_id: i32,
    pub email: String,
    pub domain: String,
    pub detail: String,
    pub created_on: chrono::DateTime<chrono::Utc>,
    pub updated_on: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct User {
    pub user_id: i32,
    pub bbb_id: String,
    pub email: String,
    pub phone: Option<String>,
    pub data: Json<serde_json::Value>,
    pub created_on: chrono::DateTime<chrono::Utc>,
    pub updated_on: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Token {
    pub token_id: i32,
    pub client_id: i32,
    pub scopes: Json<serde_json::Value>,
    pub email: String,
    pub created_on: chrono::DateTime<chrono::Utc>,
    pub updated_on: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewClient {
    pub email: String,
    pub domain: String,
    pub detail: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewUser {
    pub bbb_id: String,
    pub email: String,
    pub phone: Option<String>,
    pub data: Json<serde_json::Value>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct NewToken {
    pub client_id: i32,
    pub scopes: Json<serde_json::Value>,
    pub email: String
}