use std::{path::PathBuf, sync::Arc};

use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, warn};
#[derive(Debug)]
struct HttpServerState {
    path: PathBuf,
}
pub async fn process_http_serve(opts: PathBuf, port: u16) -> anyhow::Result<()> {
    info!("Serving {} on port {}", opts.display(), port);
    let state = HttpServerState { path: opts };
    let serve_dir = ServeDir::new(state.path.join("fixtures"))
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate();
    let router = Router::new()
        .route("/", get(file_handler))
        .route("/{*key}", get(file_handler))
        // 使用arc进行轻量级clone
        .nest_service("/tower/", serve_dir)
        .with_state(Arc::new(state));

    let server = TcpListener::bind(&format!("0.0.0.0:{}", port)).await?;
    axum::serve(server, router).await?;
    Ok(())
}

use axum::extract::State;

/// 设置为可选匹配空路径
async fn file_handler(
    State(state): State<Arc<HttpServerState>>,
    path: Option<Path<String>>,
) -> impl IntoResponse {
    let path = path.map(|p| p.0).unwrap_or_default();
    // 兼容尾部斜杠
    // /之后的访问地址路径
    let path_str = path.trim_end_matches('/');
    info!("path_str {:?}", path_str);
    // 路径文件句柄
    let p: PathBuf = std::path::Path::new(&state.path).join(path_str);
    info!("read {:?}", p);
    if !p.exists() {
        (StatusCode::NOT_FOUND, "404 Not Found".to_string()).into_response()
    } else if p.is_dir() {
        match tokio::fs::read_dir(&p).await {
            Ok(mut dir_entries) => {
                let mut html = "<html><body><ul>\n".to_string();
                // 添加返回上一级目录的链接（如果不是根目录）
                let is_root = p == state.path;
                if !is_root {
                    let parent_path = path_str
                        .rfind('/')
                        .map(|pos| path_str[..pos].to_string())
                        .unwrap_or_default();
                    info!("parent_path {:?}", parent_path);
                    html.push_str(&format!("<li><a href=\"/{}\">..</a></li>\n", parent_path));
                }
                info!("html {:#?}", html);
                // 遍历目录条目
                while let Ok(Some(entry)) = dir_entries.next_entry().await {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    let entry_path = p.join(&file_name);
                    // 构建访问子文件URL路径
                    let relative_path = format!("{}/{}", path_str, file_name);

                    // 检查是否为目录，如果是则添加斜杠
                    let display_name = if entry_path.is_dir() {
                        format!("{}/", file_name)
                    } else {
                        file_name
                    };
                    let root_slash = if is_root { "" } else { "/" };
                    html.push_str(&format!(
                        "<li><a href=\"{}{}\">{}</a></li>\n",
                        root_slash, relative_path, display_name
                    ));
                }
                html.push_str("</ul></body></html>");
                (StatusCode::OK, Html(html)).into_response()
            }
            Err(e) => {
                warn!("Error reading directory {:?}: {}", &p, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to read directory: {}", e),
                )
                    .into_response()
            }
        }
    } else {
        match tokio::fs::read_to_string(&p).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => {
                warn!("Error reading file {:?}: {}", &p, e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
        .into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::extract::Path;
    use tokio::fs;

    use super::*;
    #[tokio::test]
    async fn test_index_handler() -> anyhow::Result<()> {
        let state = Arc::new(HttpServerState {
            path: "./fixtures".into(),
        });
        let response = file_handler(State(state.clone()), Some(Path("index.html".into())))
            .await
            .into_response();
        // 1. 检查状态码
        assert_eq!(response.status(), StatusCode::OK);
        // 2. 提取 Body 并转换为字节
        // 注意：to_bytes(body, usize::MAX) 需要限制最大长度，防止内存溢出攻击测试
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
        let body_str = String::from_utf8(body_bytes.to_vec())?;

        let expected = fs::read_to_string("./fixtures/index.html").await?;
        assert_eq!(body_str, expected);
        Ok(())
    }
}
