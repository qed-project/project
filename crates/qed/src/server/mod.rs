use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

mod about;
mod auth;
mod legal;
mod routes;
mod search;
mod templates;

pub async fn run() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    info!("Starting web server on `{}:{}`", addr.ip(), addr.port());

    let app = routes::build_routes()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(2)));

    let listener = TcpListener::bind(addr)
        .await
        .expect("error binding socket for http server");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
