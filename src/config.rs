#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_name: String,
    pub jwt_user_secret: String,
    pub jwt_service_secret: String,
    pub jwt_user_max_age: i32,
    pub password_salt: String,
    pub root_user_password: String,
    pub auto_update_root_user: bool,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        let jwt_user_secret = std::env::var("JWT_USER_SECRET").expect("JWT_SECRET must be set");
        let jwt_service_secret =
            std::env::var("JWT_SERVICE_SECRET").expect("JWT_SERVICE_SECRET must be set");
        let jwt_user_max_age =
            std::env::var("JWT_USER_MAX_AGE").expect("JWT_USER_MAX_AGE must be set");
        let password_salt = std::env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set");
        let root_user_password =
            std::env::var("ROOT_USER_PASSWORD").expect("ROOT_USER_PASSWORD must be set");
        let auto_update_root_user =
            std::env::var("AUTO_UPDATE_ROOT_USER") == Ok(String::from("true"));

        Config {
            database_url,
            database_name,
            jwt_user_secret,
            jwt_service_secret,
            jwt_user_max_age: jwt_user_max_age.parse::<i32>().unwrap(),
            password_salt: password_salt,
            root_user_password,
            auto_update_root_user,
        }
    }
}
