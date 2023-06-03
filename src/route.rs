use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
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
            "/change_password",
            post(handlers::user::change_password::change_password_handler),
        )
        .route(
            "/get_permissions",
            post(handlers::user::get_permissions::get_permissions_handler),
        )
        .route(
            "/get_services",
            post(handlers::user::get_services::get_services_handler),
        )
        // Admin user
        .route(
            "/add_user",
            post(handlers::user::admin::add_user::add_user_handler),
        )
        .route(
            "/delete_user",
            delete(handlers::user::admin::delete_user::delete_user_handler),
        )
        .route(
            "/set_user_permissions",
            post(handlers::user::admin::set_user_permissions::set_user_permissions_handler),
        )
        .route(
            "/get_users",
            post(handlers::user::admin::get_users::get_users_handler),
        )
        .route(
            "/create_service",
            post(handlers::user::admin::create_service::create_service_handler),
        )
        .route(
            "/edit_service",
            post(handlers::user::admin::edit_service::edit_service_handler),
        )
        .route(
            "/delete_service",
            delete(handlers::user::admin::delete_service::delete_service_handler),
        )
        .route(
            "/add_type",
            post(handlers::user::admin::add_type::add_type_handler),
        )
        .route(
            "/edit_type",
            post(handlers::user::admin::edit_type::edit_type_handler),
        )
        .route(
            "/delete_type",
            delete(handlers::user::admin::delete_type::delete_type_handler),
        )
        .route(
            "/get_types",
            post(handlers::user::admin::get_types::get_types_handler),
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
