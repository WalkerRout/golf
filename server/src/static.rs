use axum::http::{header, HeaderValue};
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use axum::Router;

use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

use tower_layer::Layer;

/// Statically serve everything under `static/` on disk,
/// with a Cache‑Control: public, max-age=31536000, immutable header.
pub fn asset_router() -> Router {
  let svc = ServeDir::new("static")
    .precompressed_gzip()
    .precompressed_br();

  let svc = SetResponseHeaderLayer::if_not_present(
    header::CACHE_CONTROL,
    HeaderValue::from_static("public, max-age=31536000, immutable"),
  )
  .layer(svc);

  Router::new()
    .nest_service("/static", svc)
    .route("/robots.txt", get(robots))
}

async fn robots() -> impl IntoResponse {
  Redirect::permanent("/static/robots.txt")
}
