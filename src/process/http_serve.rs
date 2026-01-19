use std::{path::PathBuf, sync::Arc};

use axum::{Router, extract::Path, http::StatusCode, routing::get};
use tokio::net::TcpListener;
use tracing::{info, warn};
#[derive(Debug)]
struct HttpServerState {
    path: PathBuf,
}
pub async fn process_http_serve(opts: PathBuf, port: u16) -> anyhow::Result<()> {
    info!("Serving {} on port {}", opts.display(), port);
    let state = HttpServerState { path: opts };
    let router = Router::new()
        .route("/", get(index_handler))
        .route("/{*key}", get(index_handler))
        // 使用arc进行轻量级clone
        .with_state(Arc::new(state));

    let server = TcpListener::bind(&format!("0.0.0.0:{}", port)).await?;
    axum::serve(server, router).await?;
    Ok(())
}

use axum::extract::State;

/// 设置为可选匹配空路径
async fn index_handler(
    State(state): State<Arc<HttpServerState>>,
    path: Option<Path<String>>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path.map(|p| p.0).unwrap_or_default());
    info!("read {:?}", p);
    if !p.exists() {
        (StatusCode::NOT_FOUND, "404 Not Found".to_string())
    } else {
        match tokio::fs::read_to_string(&p).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => {
                warn!("Error reading file {:?}: {}", &p, e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
