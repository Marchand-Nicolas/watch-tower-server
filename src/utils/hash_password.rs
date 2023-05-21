use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

use crate::config::Config;

pub fn hash_password(password: String) -> String {
    let config = Config::init();
    let salt = SaltString::encode_b64(&config.password_salt.as_bytes()).unwrap();
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    return password_hash;
}
