mod api;
mod fs;
mod indexing;

use std::{env, net::SocketAddr, path::Path, sync::Arc, time::Duration};

use axum::{Router, http::Method, routing::get};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    api::AppState,
    indexing::{index_documents, rebuild_index_task},
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let serve_dir = args
        .get(1)
        .expect("path to the public directory is not specified");
    let port_str = args.get(2).expect("port is not specified");
    let port: u16 = port_str.parse().expect("invalid port number");
    let root_path = Path::new(serve_dir);

    let cors_layer = CorsLayer::new()
        .allow_origin(Any) // Open access to selected route
        .allow_methods(vec![Method::GET]);

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let index = index_documents(&root_path).expect("fail to create index");
    let state = AppState {
        root_path: Arc::new(root_path.to_path_buf()),
        index: Arc::new(RwLock::new(index)),
    };
    rebuild_index_task(state.clone(), Duration::from_mins(1));

    let router = Router::new()
        .nest_service("/assets", ServeDir::new("dist/assets"))
        .nest_service("/browse", ServeDir::new(serve_dir))
        .route("/file/{vzis}", get(api::get_file_by_vzis))
        .route("/api/ls", get(api::list_dir))
        .fallback_service(ServeFile::new("./dist/index.html"))
        .layer(ServiceBuilder::new().layer(cors_layer))
        .with_state(state);

    tokio::join!(serve(router, port));
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .expect("fail to start axum server");
}
