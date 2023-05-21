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
        .route("/login", post(handlers::user::login::login_handler))
        .route(
            "/check_auth_token",
            post(handlers::user::check_auth_token::check_auth_token_handler),
        )
        .route(
            "/add_user",
            post(handlers::user::add_user::add_user_handler),
        )
        .route(
            "/set_user_permissions",
            post(handlers::user::set_user_permissions::set_user_permissions_handler),
        )
        .with_state(app_state)
}
