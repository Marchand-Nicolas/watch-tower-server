use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, Document};

use crate::{structs, utils::check_auth_token::check_auth_token, AppState};

pub async fn get_services_handler(
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

    // get from mongodb
    let services: Vec<structs::Service> = get_services(app_state).await.unwrap();

    return Json(serde_json::json!({
        "status": "success",
        "services": services,
    }));
}

async fn get_services(
    app_state: Arc<AppState>,
) -> Result<Vec<structs::Service>, mongodb::error::Error> {
    let db = &app_state.db;
    let collection: mongodb::Collection<Document> = db.collection("services");

    let mut cursor = collection.find(doc! {}, None).await?;

    let mut result: Vec<structs::Service> = Vec::new();
    while cursor.advance().await? {
        let doc = cursor.current();
        let _id = doc.get("_id").unwrap().unwrap().as_object_id().unwrap();
        let app_name = doc.get("app_name").unwrap().unwrap().as_str().unwrap();
        let service = structs::Service {
            _id: Some(_id.to_hex()),
            app_name: Some(app_name.to_string()),
        };
        result.push(service);
    }

    return Ok(result);
}
