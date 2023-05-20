use std::sync::Arc;

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::doc;
use serde::Deserialize;

use crate::{config::Config, utils::check_auth_token::check_auth_token, AppState};

#[derive(Deserialize)]
pub struct AddUserInput {
    token: String,
    username: String,
    password: String,
}

pub async fn add_user_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<AddUserInput>,
) -> impl IntoResponse {
    let token = body.token;
    let valid = check_auth_token(token);
    if !valid {
        let json_response = serde_json::json!({
            "status": "error",
            "message": "Invalid token or token expired"
        });

        return Json(json_response);
    }

    let has_perm = has_permission(user_id, permission);

    let username = body.username;
    let password = body.password;
    // hash password
    let config = Config::init();
    let salt = SaltString::encode_b64(&config.password_salt.as_bytes()).unwrap();
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    // insert into mongodb
    let db = &app_state.db;
    let user = doc! { "username": username, "password": password_hash, "permissions": [] };
    db.collection("users").insert_one(user, None).await.unwrap();

    return Json(serde_json::json!({
        "status": "success",
    }));
}
