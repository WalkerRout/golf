use askama::Template;

use tracing::{info, warn};

use crate::external::feed::{self, Post};

#[derive(Clone)]
pub struct TemplatePost {
  pub id: usize,
  pub title: String,
  pub summary: String,
  /// If Some, use srcdoc attribute
  pub content_html: Option<String>,
  /// If Some and content_html is None, use src attribute
  pub link: Option<String>,
  pub published: String,
  pub source: String,
}

impl TemplatePost {
  fn from_post(post: Post, id: usize) -> Self {
    let published = post
      .published
      .map(|d| d.format("%b %d, %Y").to_string())
      .unwrap_or_else(|| "Unknown date".to_string());

    Self {
      id,
      title: post.title,
      summary: post.summary,
      content_html: post.content_html,
      link: post.link,
      published,
      source: post.source,
    }
  }
}

#[derive(Template, Clone)]
#[template(path = "feed.html")]
pub struct Feed {
  pub posts: Vec<TemplatePost>,
}

pub async fn build_template() -> Feed {
  // TODO: Make this configurable via env or config file
  let feed_urls = &[
    "https://blog.rust-lang.org/feed.xml",
    "https://fasterthanli.me/index.xml",
  ];

  match feed::fetch_feeds(feed_urls).await {
    Ok(posts) => {
      info!("feed template built with {} posts", posts.len());
      Feed {
        posts: posts
          .into_iter()
          .enumerate()
          .map(|(i, p)| TemplatePost::from_post(p, i))
          .collect(),
      }
    }
    Err(e) => {
      warn!("failed to fetch feeds: {e}");
      Feed { posts: Vec::new() }
    }
  }
}

#[derive(Template)]
#[template(path = "feed_content.html")]
pub struct FeedContentWrapper<'a> {
  pub title: &'a str,
  pub content: &'a str,
}
