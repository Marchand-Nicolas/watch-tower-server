use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{handler::health_checker_handler, handler::login, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health_checker", get(health_checker_handler))
        .route(
            "/login",
            post(login).route_layer(middleware::AddExtension::new(app_state.clone())),
        )
        .with_state(app_state)
}
