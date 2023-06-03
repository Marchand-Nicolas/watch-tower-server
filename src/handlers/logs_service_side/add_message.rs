use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};
use serde::Deserialize;

use crate::{utils::logs_service_side::check_service_token::check_service_token, AppState};

#[derive(Deserialize)]
pub struct LogInput {
    _id: Option<String>,
    app_id: Option<String>,
    r#type: Option<String>,
    message: String,
    timestamp: Option<i64>,
}

#[derive(Deserialize)]
pub struct AddMessageInput {
    token: String,
    log: LogInput,
}

pub async fn add_message_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<AddMessageInput>,
) -> impl IntoResponse {
    let token = body.token;

    let valid = check_service_token(app_state.clone(), token.clone()).await;

    if !valid {
        let json_response = serde_json::json!({
            "status": "error",
            "message": "Invalid token or token expired",
            "error_code": "invalid_token"
        });

        return Json(json_response);
    }

    let mut log = body.log;

    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("logs");

    if log.timestamp.is_none() {
        let current_date = chrono::Utc::now();
        let timestamp = current_date.timestamp_millis();
        log.timestamp = Some(timestamp);
    }

    if log.r#type.is_none() {
        log.r#type = Some("default".to_string());
    }

    collection
        .insert_one(
            doc! {
                "app_id": log.app_id,
                "timestamp": log.timestamp,
                "type_": log.r#type,
                "message": log.message,
            },
            None,
        )
        .await
        .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
    });

    return Json(json_response);
}
