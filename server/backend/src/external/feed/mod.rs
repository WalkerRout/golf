use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;

use chrono::{DateTime, Utc};

use feed_rs::model::{Entry, Feed};
use feed_rs::parser;

use futures::future::join_all;

use reqwest::Client;

use serde::{Deserialize, Serialize};

use tokio::sync::RwLock;
use tokio::task::AbortHandle;
use tokio::time::sleep;

use tracing::{info, warn};

use url::Url;

#[derive(Deserialize)]
struct FeedConfig {
  include: Vec<String>,
}

const FEED_CONFIG: &str = include_str!("/app/config/feed.json");

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("failed to fetch feed - {0}")]
  FetchFailed(#[from] reqwest::Error),

  #[error("failed to parse feed - {0}")]
  ParseFailed(#[from] feed_rs::parser::ParseFeedError),

  #[error("invalid config - {0}")]
  InvalidConfig(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct Post {
  pub title: String,
  pub summary: String,
  pub link: Option<String>,
  pub published: Option<DateTime<Utc>>,
  pub source: String,
}

impl Post {
  fn from_entry(entry: Entry, source_title: &str, feed_url: &Url) -> Self {
    let title = entry
      .title
      .map(|t| t.content.clone())
      .unwrap_or_else(|| "Untitled".to_string());

    let summary = entry
      .summary
      .map(|t| t.content)
      .or_else(|| entry.content.as_ref().and_then(|c| c.body.clone()))
      .map(|raw| truncate_plain_text(&strip_html_tags(&raw), 200))
      .unwrap_or_else(|| "No summary available".to_string());

    // resolve relative urls against the feed url
    let link = entry
      .links
      .first()
      .and_then(|l| feed_url.join(&l.href).ok().map(|u| u.to_string()));
    let published = entry.published.or(entry.updated);

    Self {
      title,
      summary,
      link,
      published,
      source: source_title.to_string(),
    }
  }
}

#[derive(Serialize)]
pub struct FeedPageResponse {
  pub posts: Vec<PostMeta>,
  pub total: usize,
  pub page: usize,
  pub per_page: usize,
  pub total_pages: usize,
}

#[derive(Serialize)]
pub struct PostMeta {
  pub id: usize,
  pub title: String,
  pub summary: String,
  pub source: String,
  pub published: String,
  pub link: Option<String>,
}

impl PostMeta {
  fn from_post(post: &Post, id: usize) -> Self {
    let published = post
      .published
      .map(|d| d.format("%b %d, %Y").to_string())
      .unwrap_or_else(|| "Unknown date".to_string());

    Self {
      id,
      title: post.title.clone(),
      summary: post.summary.clone(),
      source: post.source.clone(),
      published,
      link: post.link.clone(),
    }
  }
}

struct FeedCacheInner {
  posts: RwLock<Vec<Post>>,
  refresh_abort: OnceLock<AbortHandle>,
}

impl FeedCacheInner {
  async fn refresh(&self) {
    match fetch_all().await {
      Ok(posts) => {
        info!("feed cache refreshed with {} posts", posts.len());
        *self.posts.write().await = posts;
      }
      Err(e) => {
        warn!("failed to refresh feed cache - {e}");
      }
    }
  }
}

impl Drop for FeedCacheInner {
  fn drop(&mut self) {
    if let Some(handle) = self.refresh_abort.get() {
      handle.abort();
    }
  }
}

#[derive(Clone)]
pub struct FeedCache {
  inner: Arc<FeedCacheInner>,
}

impl FeedCache {
  pub async fn new() -> Self {
    let inner = Arc::new(FeedCacheInner {
      posts: RwLock::new(Vec::new()),
      refresh_abort: OnceLock::new(),
    });

    inner.refresh().await;

    let weak = Arc::downgrade(&inner);
    let handle = tokio::spawn(async move {
      loop {
        sleep(Duration::from_secs(30 * 60)).await;
        match weak.upgrade() {
          Some(inner) => inner.refresh().await,
          None => break,
        }
      }
    });

    let _ = inner.refresh_abort.set(handle.abort_handle());

    Self { inner }
  }

  pub async fn get_page(&self, page: usize, per_page: usize) -> FeedPageResponse {
    let posts = self.inner.posts.read().await;
    let total = posts.len();
    let total_pages = if total == 0 {
      1
    } else {
      (total + per_page - 1) / per_page
    };

    let page = page.clamp(1, total_pages);
    let start = (page - 1) * per_page;
    let end = (start + per_page).min(total);

    let page_posts = posts[start..end]
      .iter()
      .enumerate()
      .map(|(i, p)| PostMeta::from_post(p, start + i))
      .collect();

    FeedPageResponse {
      posts: page_posts,
      total,
      page,
      per_page,
      total_pages,
    }
  }
}

async fn fetch_feed(client: &Client, url: &str) -> Result<Feed, Error> {
  let response = client.get(url).send().await?;
  let bytes = response.bytes().await?;
  let feed = parser::parse(&bytes[..])?;
  Ok(feed)
}

async fn fetch_all() -> Result<Vec<Post>, Error> {
  let config: FeedConfig = serde_json::from_str(FEED_CONFIG)?;
  let client = Client::builder().user_agent("golf-server").build()?;

  let futures: Vec<_> = config
    .include
    .iter()
    .map(|url| {
      let client = client.clone();
      let url = url.clone();
      async move {
        let feed_url = match Url::parse(&url) {
          Ok(u) => u,
          Err(_) => {
            warn!("invalid feed URL: {}", url);
            return Vec::new();
          }
        };

        match fetch_feed(&client, &url).await {
          Ok(feed) => {
            let source_title = feed
              .title
              .map(|t| t.content.clone())
              .unwrap_or_else(|| url.to_string());

            feed
              .entries
              .into_iter()
              .map(|entry| Post::from_entry(entry, &source_title, &feed_url))
              .collect()
          }
          Err(e) => {
            warn!("failed to fetch feed {} - {}", url, e);
            Vec::new()
          }
        }
      }
    })
    .collect();

  let results = join_all(futures).await;
  let mut all_posts: Vec<Post> = results.into_iter().flatten().collect();

  all_posts.sort_by(|a, b| match (&b.published, &a.published) {
    (Some(b_date), Some(a_date)) => b_date.cmp(a_date),
    (Some(_), None) => Ordering::Greater,
    (None, Some(_)) => Ordering::Less,
    (None, None) => Ordering::Equal,
  });

  Ok(all_posts)
}

fn strip_html_tags(html: &str) -> String {
  let mut result = String::with_capacity(html.len());
  let mut in_tag = false;

  for c in html.chars() {
    match c {
      '<' => in_tag = true,
      '>' => in_tag = false,
      _ if !in_tag => result.push(c),
      _ => {}
    }
  }

  result.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate_plain_text(text: &str, max_chars: usize) -> String {
  let char_count = text.chars().count();
  if char_count <= max_chars {
    text.to_string()
  } else {
    let truncated: String = text.chars().take(max_chars - 3).collect();
    format!("{}...", truncated)
  }
}
