use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::{doc, Document};
use serde::Deserialize;

use crate::{
    config, structs,
    utils::{
        check_auth_token::check_auth_token, get_token_data::get_token_data,
        has_permission::has_permission,
    },
    AppState,
};

#[derive(Deserialize)]
pub struct RegenerateServiceTokenInput {
    token: String,
    app_id: String,
}

pub async fn regenerate_service_token_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<RegenerateServiceTokenInput>,
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

    let app_id = body.app_id;

    let config = config::Config::init();
    let jwt_secret = config.jwt_service_secret;

    let date = chrono::Utc::now().timestamp();

    // Expire in 10 years
    let exp = date + 60 * 60 * 24 * 365 * 10;
    let claims = structs::JwtServiceClaims {
        app_id: app_id.clone(),
        exp: exp as usize,
        timestamp: date,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("expired_tokens");
    // Chech if token alteady exists
    let token_doc: Option<Document> = collection
        .find_one(doc! { "app_id": app_id.clone() }, None)
        .await
        .unwrap();

    if token_doc.is_none() {
        collection
            .insert_one(doc! { "app_id": app_id, "expired_date": date - 1 }, None)
            .await
            .unwrap();
    } else {
        collection
            .update_one(
                doc! { "app_id": app_id },
                doc! { "$set": { "expired_date": date - 1 } },
                None,
            )
            .await
            .unwrap();
    }

    return Json(serde_json::json!({
        "status": "success",
        "token": token,
    }));
}
