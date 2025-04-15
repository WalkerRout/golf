use axum::body::Body;
use axum::extract::{OriginalUri, Path};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;

use include_dir::{include_dir, Dir};

use mime_guess::from_path;

use tracing::{info, warn};

use crate::template::{Cv, Error404, Home, HtmlTemplate};

pub fn rest_router() -> Router {
  Router::new()
    .route("/", get(home))
    .route("/cv", get(cv))
    .route("/static/{*path}", get(serve_static))
    .fallback(error_404)
}

async fn home() -> impl IntoResponse {
  HtmlTemplate::from(Home {
    name: "Walker Rout".into(),
  })
}

async fn cv() -> impl IntoResponse {
  HtmlTemplate::from(Cv)
}

async fn error_404(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
  warn!("unable to find resource: {}", uri);
  HtmlTemplate::from(Error404)
}

/// Statically embed our static files...
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

async fn serve_static(Path(path): Path<String>) -> impl IntoResponse {
  let path = path.trim_start_matches('/');
  if let Some(file) = STATIC_DIR.get_file(path) {
    let mime = from_path(path).first_or_octet_stream();
    let body = Body::from(file.contents());
    Response::builder()
      .status(StatusCode::OK)
      .header(header::CONTENT_TYPE, mime.to_string())
      .body(body)
      .unwrap()
  } else {
    warn!("static dir does not contain file: {}", path);
    Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(Body::empty())
      .unwrap()
  }
}
