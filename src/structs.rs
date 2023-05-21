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

pub struct Log {
    pub _id: String,
    pub app_id: String,
    pub type_: String,
    pub message: String,
}
