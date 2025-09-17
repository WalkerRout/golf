use axum::Router;
use axum::extract::OriginalUri;
use axum::response::IntoResponse;
use axum::routing::get;

use tokio::sync::OnceCell;

use tower_http::compression::CompressionLayer;

use tracing::{info, warn};

use crate::build::Build;
use crate::build::congeries;
use crate::build::home;

use crate::service::github;

use crate::template::HtmlTemplate;
use crate::template::congeries::Congeries;
use crate::template::error::Error404;
use crate::template::home::Home;

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
  // home model builds into the `Home` template
  let home = home::builder().build::<Home>();
  HtmlTemplate::from(home)
}

async fn congeries() -> impl IntoResponse {
  // singleton cache
  static CONGERIES_CELL: OnceCell<Congeries> = OnceCell::const_new();
  let congeries = CONGERIES_CELL
    .get_or_init(|| async {
      match github::fetch_repositories().await {
        Ok(congeries) => {
          info!("congeries template cached");
          congeries.into()
        }
        Err(e) => {
          warn!("failed to fetch github repos: {e}");
          congeries::builder().build()
        }
      }
    })
    .await
    .clone();
  HtmlTemplate::from(congeries)
}

async fn error_404(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
  warn!("unable to find resource: {uri}");
  HtmlTemplate::from(Error404)
}
