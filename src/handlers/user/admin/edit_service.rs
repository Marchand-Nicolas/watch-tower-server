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
pub struct EditSericeInput {
    token: String,
    app_id: String,
    new_app_name: String,
}

pub async fn edit_service_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<EditSericeInput>,
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

    let app_id = mongodb::bson::oid::ObjectId::parse_str(&body.app_id).unwrap();
    let new_app_name = body.new_app_name;

    // update mongodb
    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("services");
    collection
        .update_one(
            doc! { "_id": app_id },
            doc! { "$set": { "app_name": new_app_name } },
            None,
        )
        .await
        .unwrap();

    return Json(serde_json::json!({
        "status": "success",
    }));
}
