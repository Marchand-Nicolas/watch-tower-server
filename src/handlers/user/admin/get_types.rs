use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};

use crate::{
    structs,
    utils::{
        check_auth_token::check_auth_token, get_token_data::get_token_data,
        has_permission::has_permission,
    },
    AppState,
};

pub async fn get_types_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<structs::AuthTokenJSON>,
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

    // get from mongodb
    let types: Vec<structs::Type> = get_types(app_state).await.unwrap();

    return Json(serde_json::json!({
        "status": "success",
        "types": types,
    }));
}

async fn get_types(app_state: Arc<AppState>) -> Result<Vec<structs::Type>, mongodb::error::Error> {
    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("types");

    let mut cursor = collection.find(doc! {}, None).await?;

    let mut result: Vec<structs::Type> = Vec::new();
    while cursor.advance().await? {
        let doc = cursor.current();
        let _id = doc.get("_id").unwrap().unwrap().as_object_id().unwrap();
        let name = doc.get("name").unwrap().unwrap().as_str().unwrap();
        let color = doc.get("color").unwrap().unwrap().as_str().unwrap();
        let icon = doc.get("icon").unwrap().unwrap().as_str().unwrap();
        let importance = doc.get("importance").unwrap().unwrap().as_i32().unwrap();
        let notifications = doc
            .get("notifications")
            .unwrap()
            .unwrap()
            .as_array()
            .unwrap();
        let notifications_cursor = notifications.into_iter();
        let notifcations: Vec<String> = notifications_cursor
            .map(|notification| notification.unwrap().as_str().unwrap().to_string())
            .collect();

        let type_ = structs::Type {
            _id: Some(_id.to_hex()),
            name: name.to_string(),
            color: color.to_string(),
            icon: icon.to_string(),
            importance: importance,
            notifications: notifcations,
        };
        result.push(type_);
    }

    return Ok(result);
}
