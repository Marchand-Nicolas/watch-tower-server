use std::sync::Arc;

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

use axum::{extract::State, response::IntoResponse, Json};
use rand_core::RngCore;
use serde::Deserialize;

use crate::config::Config;

use crate::AppState;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Server is running";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}
pub async fn login(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginInput>,
) -> impl IntoResponse {
    let config = Config::init();

    let username = body.username;
    let password = body.password;
    // hash password
    let salt = SaltString::encode_b64(&config.password_salt.as_bytes()).unwrap();
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    println!("hash: {}", password_hash);
    // check in db

    let jwt_secret = &config.jwt_secret;
    let jwt_user_maxage = &config.jwt_user_maxage;

    let mut rng = rand_core::OsRng;
    let mut token = [0u8; 32];
    rng.fill_bytes(&mut token);

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &token,
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "token": token,
        "max_age": jwt_user_maxage
    });

    Json(json_response)
}

// POST
// In : app_id
// Out: token
#[derive(Deserialize)]
pub struct Encode_Token_Input {
    app_id: String,
    auth: String,
}
pub async fn encode_token_handler(Json(payload): Json<Encode_Token_Input>) -> impl IntoResponse {
    if payload.app_id == "1234567890" {
        let config = Config::init();
        let jwt_secret = &config.jwt_secret;

        let seed = rand_core::OsRng.next_u64().to_string();

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &seed,
            &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .unwrap();

        let json_response = serde_json::json!({
            "status": "success",
            "token": token,
        });

        Json(json_response)
    } else {
        let json_response = serde_json::json!(
            {
                "status": "error",
                "message": "Invalid JSON"
            }
        );

        Json(json_response)
    }
}
