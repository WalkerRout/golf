use axum::Router;
use axum::extract::OriginalUri;
use axum::response::IntoResponse;
use axum::routing::get;

use tokio::sync::OnceCell;

use tower_http::compression::CompressionLayer;

use tracing::warn;

use crate::state::AppState;
use crate::template::HtmlTemplate;
use crate::template::about;
use crate::template::congeries::{self, Congeries};
use crate::template::error::Error404;
use crate::template::feed::Feed;
use crate::template::home;

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/", get(home))
    .route("/about", get(about))
    .route("/congeries", get(congeries))
    .route("/feed", get(feed))
    .fallback(error_404)
    .layer(CompressionLayer::new().br(true).gzip(true))
}

async fn home() -> impl IntoResponse {
  let home = home::build_template().await;
  HtmlTemplate::from(home)
}

async fn about() -> impl IntoResponse {
  let about = about::build_template().await;
  HtmlTemplate::from(about)
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

async fn feed() -> impl IntoResponse {
  let feed = feed::build_template().await;
  HtmlTemplate::from(feed)
}

async fn error_404(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
  warn!("unable to find resource: {uri}");
  HtmlTemplate::from(Error404)
}
