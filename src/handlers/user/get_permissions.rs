use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};

use crate::{
    structs,
    utils::{check_auth_token::check_auth_token, get_token_data::get_token_data},
    AppState,
};

pub async fn get_permissions_handler(
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

    let token_data = get_token_data(token.clone());
    let user_id = mongodb::bson::oid::ObjectId::parse_str(&token_data.user_id).unwrap();

    // get from mongodb
    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("users");
    let user = collection
        .find_one(doc! { "_id": user_id }, None)
        .await
        .unwrap()
        .unwrap();
    let permissions = user.get("permissions").unwrap().clone();
    return Json(serde_json::json!({
        "status": "success",
        "permissions": permissions,
    }));
}
