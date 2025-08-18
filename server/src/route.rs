use axum::extract::OriginalUri;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

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
  rest_router().merge(r#static::asset_router())
}

fn rest_router() -> Router {
  Router::new()
    .route("/", get(home))
    .route("/cv", get(cv))
    .fallback(error_404)
    .layer(CompressionLayer::new().br(true).gzip(true))
}

async fn home() -> impl IntoResponse {
  let home = home::builder().build::<Home>();
  HtmlTemplate::from(home)
}

async fn cv() -> impl IntoResponse {
  let cv = cv::builder().build::<Cv>();
  HtmlTemplate::from(cv)
}

async fn error_404(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
  warn!("unable to find resource: {}", uri);
  HtmlTemplate::from(Error404)
}
