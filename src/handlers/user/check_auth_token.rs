use axum::{response::IntoResponse, Json};

use crate::{structs::AuthTokenJSON, utils::check_auth_token::check_auth_token};

pub async fn check_auth_token_handler(Json(body): Json<AuthTokenJSON>) -> impl IntoResponse {
    let token = body.token;
    let valid = check_auth_token(token);

    if valid {
        let json_response = serde_json::json!({
            "status": "success",
        });

        return Json(json_response);
    }
    let json_response = serde_json::json!({
        "status": "error",
        "message": "Invalid token or token expired",
        "error_code": "invalid_token"
    });

    return Json(json_response);
}
