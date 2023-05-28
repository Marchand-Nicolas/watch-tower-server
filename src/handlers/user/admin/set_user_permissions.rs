use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};
use serde::Deserialize;

use crate::{
    utils::{
        check_auth_token::check_auth_token, get_token_data::get_token_data,
        has_permission::has_permission,
    },
    AppState,
};

#[derive(Deserialize)]
pub struct AddUserInput {
    token: String,
    target_user_id: String,
    new_permissions: Vec<String>,
}

pub async fn set_user_permissions_handler(
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

    let target_user_id = body.target_user_id;
    let new_permissions = body.new_permissions;

    let db = &app_state.db;
    let object_id = mongodb::bson::oid::ObjectId::parse_str(&target_user_id).unwrap();
    let collection: mongodb::Collection<Document> = db.collection("users");
    collection
        .update_one(
            doc! { "_id": object_id },
            doc! { "$set": { "permissions": new_permissions } },
            None,
        )
        .await
        .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
    });

    return Json(json_response);
}
