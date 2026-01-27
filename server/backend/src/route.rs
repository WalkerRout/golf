use axum::Router;
use axum::extract::OriginalUri;
use axum::response::IntoResponse;
use axum::routing::get;

use tokio::sync::OnceCell;

use tower_http::compression::CompressionLayer;

use tracing::warn;

use crate::template::HtmlTemplate;
use crate::template::congeries::{self, Congeries};
use crate::template::error::Error404;
use crate::template::home;

use crate::r#static;

pub fn router() -> Router {
  rest_router().merge(r#static::asset_router())
}

fn rest_router() -> Router {
  Router::new()
    .route("/", get(home))
    .route("/congeries", get(congeries))
    .fallback(error_404)
    .layer(CompressionLayer::new().br(true).gzip(true))
}

async fn home() -> impl IntoResponse {
  let home = home::build_template().await;
  HtmlTemplate::from(home)
}

async fn congeries() -> impl IntoResponse {
  // singleton cache
  static CONGERIES_CELL: OnceCell<Congeries> = OnceCell::const_new();
  let congeries = CONGERIES_CELL
    .get_or_init(|| async { congeries::build_template().await })
    .await
    .clone();
  HtmlTemplate::from(congeries)
}

async fn error_404(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
  warn!("unable to find resource: {uri}");
  HtmlTemplate::from(Error404)
}
