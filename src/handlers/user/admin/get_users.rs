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

pub async fn get_users_handler(
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
    let users: Vec<structs::User> = get_users(app_state).await.unwrap();

    return Json(serde_json::json!({
        "status": "success",
        "users": users,
    }));
}

async fn get_users(app_state: Arc<AppState>) -> Result<Vec<structs::User>, mongodb::error::Error> {
    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("users");

    let mut cursor = collection.find(doc! {}, None).await?;

    let mut result: Vec<structs::User> = Vec::new();
    while cursor.advance().await? {
        let doc = cursor.current();
        let _id = doc.get("_id").unwrap().unwrap().as_object_id().unwrap();
        let username = doc.get("username").unwrap().unwrap().as_str().unwrap();
        let permissions = doc.get("permissions").unwrap().unwrap().as_array().unwrap();
        let permission_cursor = permissions.into_iter();
        let permissions_result: Vec<String> = permission_cursor
            .map(|permission| permission.unwrap().as_str().unwrap().to_string())
            .collect();
        let user = structs::User {
            _id: Some(_id.to_hex()),
            username: username.to_string(),
            permissions: permissions_result,
            password: None,
        };
        result.push(user);
    }

    return Ok(result);
}
