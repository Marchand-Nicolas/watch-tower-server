use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{config::Config, structs::Claims};

pub fn check_auth_token(jwt_token: String) -> bool {
    let config = Config::init();
    let jwt_secret = &config.jwt_secret;

    let token_data = decode::<Claims>(
        &jwt_token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    );

    if token_data.is_err() {
        return false;
    }

    let token_data = token_data.unwrap();

    let date = chrono::Utc::now();

    if token_data.claims.exp < date.timestamp() as usize {
        return false;
    }

    return true;
}
