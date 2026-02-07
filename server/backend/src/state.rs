use crate::external::feed::FeedCache;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

#[derive(Clone)]
pub struct AppState {
  pub feed_cache: FeedCache,
}

impl AppState {
  pub async fn new() -> Result<Self, Error> {
    Ok(Self {
      feed_cache: FeedCache::new().await,
    })
  }
}
