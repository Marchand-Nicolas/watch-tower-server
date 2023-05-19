use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{handler::health_checker_handler, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(app_state)
}
