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
pub struct AddTypeInput {
    token: String,
    name: String,
}

pub async fn add_type_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<AddTypeInput>,
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

    let name = body.name;
    // insert into mongodb
    let db = app_state.db.clone();
    let user = doc! { "name": name,
         "color": "gray",
         "icon": "default",
         "importance": 0,
         "notifications": []
    };
    let res = db.collection("types").insert_one(user, None).await.unwrap();
    let type_id = res.inserted_id.as_object_id().unwrap().to_hex();

    return Json(serde_json::json!({
        "status": "success",
        "_id": type_id
    }));
}
