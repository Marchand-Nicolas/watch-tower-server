use std::sync::Arc;

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::doc;
use serde::Deserialize;

use crate::{
    config::Config,
    utils::{
        check_auth_token::check_auth_token, get_token_data::get_token_data,
        has_permission::has_permission,
    },
    AppState,
};

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
    let valid = check_auth_token(token.clone());
    if !valid {
        let json_response = serde_json::json!({
            "status": "error",
            "message": "Invalid token or token expired",
            "error_code": "invalid_token"
        });

        return Json(json_response);
    }

    let token_data = get_token_data(token);

    let has_perm = has_permission(
        token_data.user_id,
        "administrator".to_string(),
        app_state.clone(),
    )
    .await;

    if !has_perm {
        let json_response = serde_json::json!({
            "status": "error",
            "message": "You don't have administrator permission",
            "error_code": "permission_denied"
        });

        return Json(json_response);
    }

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
    let db = &app_state.db;
    // check if user already exists
    let user: Option<mongodb::bson::Document> = db
        .collection("users")
        .find_one(doc! { "username": username.clone() }, None)
        .await
        .unwrap();
    if user.is_some() {
        let json_response = serde_json::json!({
            "status": "error",
            "message": "User already exists",
            "error_code": "user_already_exists"
        });

        return Json(json_response);
    }
    // insert into mongodb
    let user = doc! { "username": username, "password": password_hash, "permissions": [] };
    db.collection("users").insert_one(user, None).await.unwrap();

    return Json(serde_json::json!({
        "status": "success",
    }));
}
