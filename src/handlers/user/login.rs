use std::sync::Arc;

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

use axum::{extract::State, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::{doc, Document};
use serde::Deserialize;

use crate::config::Config;

use crate::structs;

use crate::AppState;

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}
pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
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
    // check in mongodb
    let db = &app_state.db;
    let user: Option<Document> = db
        .collection("users")
        .find_one(
            doc! { "username": username, "password": password_hash },
            None,
        )
        .await
        .unwrap();

    // if user is not found
    if user.is_none() {
        let json_response = serde_json::json!({
            "status": "error",
            "message": "Invalid username or password",
            "error_code": "invalid_credentials"
        });

        return Json(json_response);
    }
    // if user is found
    let jwt_secret = &config.jwt_user_secret;
    let jwt_user_max_age = &config.jwt_user_max_age;

    // get date in 'jwt_user_max_age' days
    let date = chrono::Utc::now() + chrono::Duration::days(*jwt_user_max_age as i64);

    let claims = structs::JwtUserClaims {
        exp: date.timestamp() as usize,
        user_id: user.unwrap().get_object_id("_id").unwrap().to_hex(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "token": token,
        "max_age": jwt_user_max_age
    });

    Json(json_response)
}
