use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/health_checker",
            get(handlers::health_checker::health_checker_handler),
        )
        // User
        .route("/login", post(handlers::user::login::login_handler))
        .route(
            "/check_auth_token",
            post(handlers::user::check_auth_token::check_auth_token_handler),
        )
        .route(
            "/set_user_permissions",
            post(handlers::user::set_user_permissions::set_user_permissions_handler),
        )
        .route(
            "/change_password",
            post(handlers::user::change_password::change_password_handler),
        )
        // Admin user
        .route(
            "/add_user",
            post(handlers::user::admin::add_user::add_user_handler),
        )
        // Logs user side
        .route(
            "/get_logs",
            post(handlers::logs_user_side::get_logs::get_logs_handler),
        )
        // Admin logs user side
        .route(
            "/regenerate_service_token",
            post(handlers::logs_user_side::admin::regenerate_service_token::regenerate_service_token_handler),
        )
        // Logs service side
        .route(
            "/service/add_message",
            post(handlers::logs_service_side::add_message::add_message_handler),
        )
        .with_state(app_state)
}
