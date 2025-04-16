use axum::extract::{OriginalUri, Path};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use tokio::sync::OnceCell;

use tower_http::compression::CompressionLayer;

use tracing::warn;

use crate::template::cv::Cv;
use crate::template::error::Error404;
use crate::template::home::Home;
use crate::template::HtmlTemplate;

use crate::model::Build;

use crate::data::cv;
use crate::data::home;

use crate::r#static;

pub fn router() -> Router {
  rest_router()
}

fn rest_router() -> Router {
  Router::new()
    .route("/", get(home))
    .route("/cv", get(cv))
    .route("/static/{*path}", get(r#static))
    .fallback(error_404)
    // gzip strips etag??? https://github.com/rtomayko/rack-cache/issues/111#issuecomment-92614509
    //.layer(CompressionLayer::new().br(true))
}

async fn home() -> impl IntoResponse {
  static HOME_CELL: OnceCell<Home> = OnceCell::const_new();
  let home = HOME_CELL
    .get_or_init(|| async { home::builder().build::<Home>() })
    .await
    .clone();
  HtmlTemplate::from(home)
}

async fn cv() -> impl IntoResponse {
  static CV_CELL: OnceCell<Cv> = OnceCell::const_new();
  let cv = CV_CELL
    .get_or_init(|| async { cv::builder().build::<Cv>() })
    .await
    .clone();
  HtmlTemplate::from(cv)
}

async fn error_404(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
  warn!("unable to find resource: {}", uri);
  HtmlTemplate::from(Error404)
}

async fn r#static(Path(path): Path<String>) -> impl IntoResponse {
  r#static::serve(&path).await
}
