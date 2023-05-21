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

    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("logs");

    let logs: Vec<structs::Log> = get_logs(app_state).await.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
    });

    return Json(json_response);
}

async fn get_logs(app_state: Arc<AppState>) -> Result<Vec<structs::Log>, mongodb::error::Error> {
    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("logs");

    let mut cursor = collection.find(doc! {}, None).await?;

    let mut result: Vec<structs::Log> = Vec::new();
    while cursor.advance().await? {
        let doc = cursor.current();
        println!("{:?}", doc);
    }

    return Ok(result);
}
