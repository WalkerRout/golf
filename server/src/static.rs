use axum::body::Body;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};

use bytes::Bytes;

use include_dir::{include_dir, Dir};

use mime_guess::from_path;

use tracing::warn;

/// Statically embed our static files...
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

pub async fn serve(path: &str) -> impl IntoResponse {
  let path = path.trim_start_matches('/');
  if let Some(file) = STATIC_DIR.get_file(path) {
    let mime = from_path(path).first_or_octet_stream();
    let body = Body::from(Bytes::from_static(file.contents()));
    Response::builder()
      .status(StatusCode::OK)
      .header(header::CONTENT_TYPE, mime.to_string())
      .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
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
