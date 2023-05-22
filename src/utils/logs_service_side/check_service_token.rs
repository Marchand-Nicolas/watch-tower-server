use std::sync::Arc;

use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::bson::doc;

use crate::{config::Config, structs::JwtServiceClaims, AppState};

pub async fn check_service_token(appstate: Arc<AppState>, jwt_token: String) -> bool {
    let config = Config::init();
    let jwt_secret = &config.jwt_service_secret;

    let token_data = decode::<JwtServiceClaims>(
        &jwt_token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    );

    if token_data.is_err() {
        return false;
    }

    let token_data = token_data.unwrap();
    let token_timestamp = token_data.claims.timestamp;
    let app_id = token_data.claims.app_id;

    let db = &appstate.db;
    let doc: Option<mongodb::bson::Document> = db
        .collection("expired_tokens")
        .find_one(doc! { "app_id": app_id }, None)
        .await
        .unwrap();

    if doc.is_none() {
        return true;
    }

    let last_expired_token = doc.unwrap().get_i64("expired_date").unwrap();

    if last_expired_token > token_timestamp {
        return false;
    }

    return true;
}
