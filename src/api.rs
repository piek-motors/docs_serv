use std::{path::PathBuf, sync::Arc};

use axum::body::Body;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use tokio::fs::File;
use tokio::sync::RwLock;
use tokio_util::io::ReaderStream;

use crate::{
    fs::{FsNode, read_dir_recursive},
    indexing::Index,
};

#[derive(Clone)]
pub struct AppState {
    pub root_path: Arc<PathBuf>,
    pub index: Arc<RwLock<Index>>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn list_dir(State(state): State<AppState>) -> impl IntoResponse {
    match read_dir_recursive(&state.root_path) {
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

pub async fn get_file_by_vzis(
    State(state): State<AppState>,
    Path(vzis): Path<String>,
) -> impl IntoResponse {
    let index = state.index.read().await;

    match index.get(&vzis) {
        Some(file_ref) => match File::open(&file_ref.path).await {
            Ok(file) => {
                let stream = ReaderStream::new(file);
                Body::from_stream(stream).into_response()
            }
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()> {
                    data: None,
                    error: Some(err.to_string()),
                }),
            )
                .into_response(),
        },

        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()> {
                data: None,
                error: Some("Document not found".to_string()),
            }),
        )
            .into_response(),
    }
}
