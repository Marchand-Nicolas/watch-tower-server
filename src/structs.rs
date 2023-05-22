use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtUserClaims {
    pub exp: usize,
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtServiceClaims {
    pub exp: usize,
    pub app_id: String,
    pub timestamp: i64,
}

#[derive(Deserialize)]
pub struct AuthTokenJSON {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Log {
    pub _id: Option<String>,
    pub app_id: Option<String>,
    pub type_: Option<String>,
    pub message: String,
    pub timestamp: Option<i64>,
}
