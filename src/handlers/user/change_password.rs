use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};
use serde::Deserialize;

use crate::{
    utils::{
        check_auth_token::check_auth_token, get_token_data::get_token_data,
        hash_password::hash_password,
    },
    AppState,
};

#[derive(Deserialize)]
pub struct ChangePasswordInput {
    token: String,
    new_password: String,
}

pub async fn change_password_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<ChangePasswordInput>,
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
    let user_id = token_data.user_id;

    let new_password = body.new_password;
    let password_hash = hash_password(new_password);

    let db = &app_state.db;
    let object_id = mongodb::bson::oid::ObjectId::parse_str(&user_id).unwrap();
    let collection: mongodb::Collection<Document> = db.collection("users");
    collection
        .update_one(
            doc! { "_id": object_id },
            doc! { "$set": { "password": password_hash } },
            None,
        )
        .await
        .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
    });

    return Json(json_response);
}
