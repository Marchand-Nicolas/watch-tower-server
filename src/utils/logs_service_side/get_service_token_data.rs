use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{config::Config, structs::JwtServiceClaims};

pub fn get_service_token_data(jwt_token: String) -> Option<JwtServiceClaims> {
    let config = Config::init();
    let jwt_secret = &config.jwt_service_secret;

    let token_data = decode::<JwtServiceClaims>(
        &jwt_token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    );

    if token_data.is_err() {
        return None;
    }

    let token_data = token_data.unwrap();

    let date = chrono::Utc::now();

    if token_data.claims.exp < date.timestamp() as usize {
        return None;
    }

    return Some(token_data.claims);
}
