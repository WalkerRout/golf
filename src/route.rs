use axum::extract::OriginalUri;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use tower_http::services::{ServeDir, ServeFile};

use tracing::info;

use crate::template::{Error404, Home, HtmlTemplate};

pub fn router() -> Router {
  let static_dir = ServeDir::new("static");
  Router::new()
    .route("/", get(home))
    .fallback(error_404)
    .nest_service("/static", static_dir)
}

async fn home() -> impl IntoResponse {
  HtmlTemplate::from(Home {
    name: "Walker".into(),
  })
}

async fn error_404(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
  info!("unable to find resource: {}", uri);
  HtmlTemplate::from(Error404)
}
