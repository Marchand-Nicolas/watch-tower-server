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

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub _id: Option<String>,
    pub app_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub _id: Option<String>,
    pub username: String,
    pub password: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Type {
    pub _id: Option<String>,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub importance: i32,
    pub notifications: Vec<String>,
}
