use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};
use serde::Deserialize;

use crate::{structs, utils::check_auth_token::check_auth_token, AppState};

#[derive(Deserialize)]
pub struct GetLogsInput {
    token: String,
    app_id: String,
}

pub async fn get_logs_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<GetLogsInput>,
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
    let app_id = body.app_id;
    let logs: Vec<structs::Log> = get_logs(app_state, app_id).await.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "logs": logs,
    });

    return Json(json_response);
}

async fn get_logs(
    app_state: Arc<AppState>,
    app_id: String,
) -> Result<Vec<structs::Log>, mongodb::error::Error> {
    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("logs");

    let mut cursor = collection
        .find(
            doc! {
                "app_id": app_id
            },
            None,
        )
        .await?;

    let mut result: Vec<structs::Log> = Vec::new();
    while cursor.advance().await? {
        let doc = cursor.current();
        let message = doc.get("message").unwrap().unwrap().as_str().unwrap();
        let app_id = doc.get("app_id").unwrap().unwrap().as_str().unwrap();
        let timestamp = doc.get("timestamp").unwrap().unwrap().as_i64().unwrap();
        let _id = doc.get("_id").unwrap().unwrap().as_object_id().unwrap();
        let type_ = doc.get("type_").unwrap().unwrap().as_str().unwrap();
        let log = structs::Log {
            _id: Some(_id.to_hex()),
            app_id: Some(app_id.to_string()),
            type_: Some(type_.to_string()),
            message: message.to_string(),
            timestamp: Some(timestamp),
        };
        result.push(log);
    }

    return Ok(result);
}
