use std::{env, net::SocketAddr};

use axum::Router;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let doc_path = args
        .get(1)
        .expect("path to the public directory is not specified");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .route_service("/", ServeFile::new("./dist/index.html"))
        .nest_service("/browse", ServeDir::new(doc_path))
        .nest_service("/assets", ServeDir::new("dist/assets"));

    tokio::join!(serve(router, 3000));
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .expect("fail to start axum server");
}
