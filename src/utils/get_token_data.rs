use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{config::Config, structs::JwtUserClaims};

pub fn get_token_data(jwt_token: String) -> JwtUserClaims {
    let config = Config::init();
    let jwt_secret = &config.jwt_user_secret;

    let token_data = decode::<JwtUserClaims>(
        &jwt_token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    );

    let error_object = JwtUserClaims {
        exp: 0,
        user_id: "".to_string(),
    };

    if token_data.is_err() {
        return error_object;
    }

    let token_data = token_data.unwrap();

    let date = chrono::Utc::now();

    if token_data.claims.exp < date.timestamp() as usize {
        return error_object;
    }

    return token_data.claims;
}
