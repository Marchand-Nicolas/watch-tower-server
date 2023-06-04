use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};
use serde::Deserialize;
use std::fs::File;

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

    let res = collection
        .insert_one(
            doc! {
                "app_id": log.app_id.clone(),
                "timestamp": log.timestamp.clone(),
                "type_": log.r#type.clone(),
                "message": log.message.clone(),
            },
            None,
        )
        .await
        .unwrap();

    let collection: mongodb::Collection<Document> = db.collection("services");
    let parsed_app_id =
        mongodb::bson::oid::ObjectId::parse_str(&log.app_id.clone().unwrap()).unwrap();
    let service = collection
        .find_one(
            doc! {
                "_id": parsed_app_id
            },
            None,
        )
        .await
        .unwrap()
        .unwrap();

    let colllection: mongodb::Collection<Document> = db.collection("types");
    let type_ = colllection
        .find_one(
            doc! {
                "name": log.r#type.clone().unwrap()
            },
            None,
        )
        .await
        .unwrap()
        .unwrap();

    let notifications = type_.get("notifications").unwrap().as_array().unwrap();

    if notifications.contains(&"discord".to_string().into()) {
        let config_file = File::open("config.json").unwrap();
        let config: serde_json::Value = serde_json::from_reader(config_file).unwrap();
        let discord_webhook = config["discord_webhook"].as_str().unwrap();

        let message = format!(
            "<t:{}> __{}__\n**{}**\n{}\n➡️ [open](https://watch-t.vercel.app/dashboard?page=logs&services={}#log_{})",
            log.timestamp.unwrap(),
            service.get("app_name").unwrap().as_str().unwrap(),
            log.r#type.unwrap(),
            log.message,
            log.app_id.unwrap(),
            res.inserted_id.as_object_id().unwrap().to_hex()
        );

        let client = reqwest::Client::new();
        client
            .post(discord_webhook)
            .form(&serde_json::json!({ "content": message }))
            .send()
            .await
            .unwrap();
    }

    let json_response = serde_json::json!({
        "status": "success",
    });

    return Json(json_response);
}
