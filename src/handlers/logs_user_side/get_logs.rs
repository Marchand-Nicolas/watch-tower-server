use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};
use serde::Deserialize;

use crate::{structs, utils::check_auth_token::check_auth_token, AppState};

#[derive(Deserialize)]
pub struct GetLogsInput {
    token: String,
    target_apps: Option<Vec<String>>,
    target_types: Option<Vec<String>>,
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
    let app_ids = body.target_apps;
    let types = body.target_types;
    let mut res_logs: std::collections::HashMap<String, Vec<structs::Log>> =
        std::collections::HashMap::new();
    for app_id in app_ids.unwrap() {
        let logs: Vec<structs::Log> = get_logs(app_state.clone(), app_id.clone()).await.unwrap();
        let mut logs: Vec<structs::Log> = logs
            .into_iter()
            .filter(|log| {
                if types.is_none() {
                    return true;
                }
                let types = types.clone().unwrap();
                let type_ = log.type_.clone().unwrap();
                return types.contains(&type_);
            })
            .collect();
        logs.sort_by(|a, b| {
            let a_timestamp = a.timestamp.clone().unwrap();
            let b_timestamp = b.timestamp.clone().unwrap();
            return a_timestamp.cmp(&b_timestamp);
        });
        res_logs.insert(app_id, logs);
    }

    let json_response = serde_json::json!({
        "status": "success",
        "logs": res_logs
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
