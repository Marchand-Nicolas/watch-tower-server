use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Array, Document};
use serde::Deserialize;

use crate::{
    utils::{
        check_auth_token::check_auth_token, get_token_data::get_token_data,
        has_permission::has_permission,
    },
    AppState,
};

#[derive(Deserialize)]
pub struct EditTypeInput {
    token: String,
    type_id: String,
    name: String,
    color: String,
    icon: String,
    notifications: Vec<String>,
    importance: i32,
}

pub async fn edit_type_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<EditTypeInput>,
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

    let _id = mongodb::bson::oid::ObjectId::parse_str(&body.type_id).unwrap();
    let name = body.name;
    let color = body.color;
    let icon = body.icon;
    let notifications = body.notifications;
    let importance = body.importance;

    // update mongodb
    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("types");
    collection
        .update_one(
            doc! { "_id": _id },
            doc! {
                "$set": {
                    "name": name,
                    "color": color,
                    "icon": icon,
                    "notifications": notifications,
                    "importance": importance,
                }
            },
            None,
        )
        .await
        .unwrap();

    return Json(serde_json::json!({
        "status": "success",
    }));
}
