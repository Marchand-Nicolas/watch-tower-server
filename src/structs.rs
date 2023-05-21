use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct AuthTokenJSON {
    pub token: String,
}
