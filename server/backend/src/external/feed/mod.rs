use std::cmp::Ordering;

use askama::Template;

use chrono::{DateTime, Utc};

use feed_rs::model::{Entry, Feed};
use feed_rs::parser;

use tracing::warn;

use reqwest::Client;

use crate::template::feed::FeedContentWrapper;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("failed to fetch feed - {0}")]
  FetchFailed(#[from] reqwest::Error),

  #[error("failed to parse feed - {0}")]
  ParseFailed(#[from] feed_rs::parser::ParseFeedError),
}

#[derive(Clone)]
pub struct Post {
  // post title
  pub title: String,
  // post summary/excerpt for the card view
  pub summary: String,
  // actual content (html) for the iframe srcdoc
  pub content_html: Option<String>,
  // link to the original post
  pub link: Option<String>,
  pub published: Option<DateTime<Utc>>,
  // source name
  pub source: String,
}

impl Post {
  fn from_entry(entry: Entry, source_title: &str) -> Self {
    // deconstruct the entry into nice, formatted little pieces
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

    let link = entry.links.first().map(|l| l.href.clone());
    let published = entry.published.or(entry.updated);

    // extract html for iframe, wrapped in full document
    let content_html = entry.content.and_then(|c| {
      // only wrap if body exists
      c.body.map(|body| wrap_html_content(&body, &title))
    });

    Self {
      title,
      summary,
      content_html,
      link,
      published,
      source: source_title.to_string(),
    }
  }
}

async fn fetch_feed(client: &Client, url: &str) -> Result<Feed, Error> {
  let response = client.get(url).send().await?;
  let bytes = response.bytes().await?;
  let feed = parser::parse(&bytes[..])?;
  Ok(feed)
}

pub async fn fetch_feeds(urls: &[&str]) -> Result<Vec<Post>, Error> {
  let client = Client::builder().user_agent("golf-server").build()?;

  let mut all_posts = Vec::new();

  for url in urls {
    match fetch_feed(&client, url).await {
      Ok(feed) => {
        let source_title = feed
          .title
          .map(|t| t.content.clone())
          .unwrap_or_else(|| url.to_string());

        for entry in feed.entries {
          all_posts.push(Post::from_entry(entry, &source_title));
        }
      }
      Err(e) => {
        warn!("failed to fetch feed {}: {}", url, e);
        // continue with other feeds even if one fails
      }
    }
  }

  // sort by newest publication date
  all_posts.sort_by(|a, b| {
    match (&b.published, &a.published) {
      (Some(b_date), Some(a_date)) => b_date.cmp(a_date),
      (Some(_), None) => Ordering::Less,
      (None, Some(_)) => Ordering::Greater,
      (None, None) => Ordering::Equal,
    }
  });

  Ok(all_posts)
}

// strip tags to generate plaintext summaries
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

  // collapse whitespace
  result.split_whitespace().collect::<Vec<_>>().join(" ")
}

// shrink some text and add ellipsis for effect...
fn truncate_plain_text(text: &str, max_len: usize) -> String {
  if text.len() <= max_len {
    text.to_string()
  } else {
    // find word boundary near max_len to avoid cutting words
    let truncated = &text[..max_len-3];
    format!("{}...", truncated)
  }
}

// wrap html fragment in our styled document
fn wrap_html_content(content: &str, title: &str) -> String {
  let wrapper = FeedContentWrapper { title, content };
  wrapper.render().unwrap_or_else(|e| {
    tracing::error!("failed to render feed content wrapper: {}", e);
    format!("<html><body>{}</body></html>", content)
  })
}
