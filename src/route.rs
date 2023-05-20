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
        .route("/login", post(handlers::login::login_handler))
        .route(
            "/check_auth_token",
            post(handlers::check_auth_token::check_auth_token_handler),
        )
        .route("/add_user", post(handlers::add_user::add_user_handler))
        .with_state(app_state)
}
