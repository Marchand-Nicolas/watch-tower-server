use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::doc;
use serde::Deserialize;

use crate::{
    utils::{
        check_auth_token::check_auth_token, get_token_data::get_token_data,
        has_permission::has_permission,
    },
    AppState,
};

#[derive(Deserialize)]
pub struct CreateSericeInput {
    token: String,
    app_name: String,
}

pub async fn create_service_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateSericeInput>,
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

    let app_name = body.app_name;

    // insert into mongodb
    let app = doc! { "app_name": app_name };
    let db = &app_state.db;
    let res = db
        .collection("services")
        .insert_one(app, None)
        .await
        .unwrap();
    let service_id = res.inserted_id.as_object_id().unwrap().to_hex();
    return Json(serde_json::json!({
        "status": "success",
        "_id": service_id,
    }));
}
