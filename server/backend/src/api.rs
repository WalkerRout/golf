use axum::Json;
use axum::Router;
use axum::extract::{FromRef, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;

use serde::Deserialize;

use tower_http::compression::CompressionLayer;

use crate::external::feed::{FeedCache, FeedPageResponse};
use crate::state::AppState;

#[derive(Clone)]
pub struct ApiState {
  pub feed_cache: FeedCache,
}

impl FromRef<AppState> for ApiState {
  fn from_ref(state: &AppState) -> Self {
    ApiState {
      feed_cache: state.feed_cache.clone(),
    }
  }
}

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/api/feed", get(api_feed))
    .layer(CompressionLayer::new().br(true).gzip(true))
}

#[derive(Deserialize)]
struct FeedQueryParams {
  #[serde(default = "default_page")]
  page: usize,
  #[serde(default = "default_per_page")]
  per_page: usize,
}

fn default_page() -> usize {
  1
}

fn default_per_page() -> usize {
  24
}

async fn api_feed(
  State(state): State<ApiState>,
  Query(params): Query<FeedQueryParams>,
) -> impl IntoResponse {
  let response = state
    .feed_cache
    .get_page(params.page, params.per_page)
    .await;
  Json(response)
}
