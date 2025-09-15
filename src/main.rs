use std::net::SocketAddr;

use anyhow;
use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::Embed;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    let app: Router<()> = Router::new()
        .route("/", get(home_handler))
        .route("/{*path}", get(any_file_handler));

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    _ = axum::serve(listener, app).await;

    Ok(())
}

async fn home_handler() -> impl IntoResponse {
    static_mapper("index.html".into()).await
}

async fn any_file_handler(Path(uri): Path<String>) -> impl IntoResponse {
    let uri = if uri.ends_with("/") {
        format!("{}/{}", uri, "index.html")
    } else {
        uri
    };
    static_mapper(uri).await
}

async fn static_mapper(uri: String) -> impl IntoResponse {
    tracing::info!("{uri} accessed");
    StaticFile(uri)
}

#[derive(Embed)]
#[folder = "src/assets/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}
