use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;

use crate::fs::{FsNode, read_dir_recursive};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Clone)]
struct AppState {
    serve_dir: Arc<String>,
}

async fn health() -> impl IntoResponse {
    let body = ApiResponse {
        data: Some("ok".to_string()),
        error: None,
    };
    (StatusCode::OK, Json(body))
}

async fn list_dir(State(state): State<AppState>) -> impl IntoResponse {
    let root = std::path::Path::new(state.serve_dir.as_str());

    match read_dir_recursive(root) {
        Ok(tree) => {
            let body = ApiResponse {
                data: Some(tree),
                error: None,
            };
            (StatusCode::OK, Json(body))
        }
        Err(err) => {
            let body = ApiResponse::<Vec<FsNode>> {
                data: None,
                error: Some(err.to_string()),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
        }
    }
}

pub fn router(serve_dir: String) -> Router {
    let state = AppState {
        serve_dir: Arc::new(serve_dir),
    };

    Router::new()
        .route("/health", get(health))
        .route("/ls", get(list_dir))
        .with_state(state)
}
