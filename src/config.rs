#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub jwt_user_maxage: i32,
    pub password_salt: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_user_maxage =
            std::env::var("JWT_USER_MAX_AGE").expect("JWT_USER_MAX_AGE must be set");
        let password_salt = std::env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set");
        Config {
            database_url,
            database_name,
            jwt_secret,
            jwt_user_maxage: jwt_user_maxage.parse::<i32>().unwrap(),
            password_salt: std::env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set"),
        }
    }
}
