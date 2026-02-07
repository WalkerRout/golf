use askama::Template;

#[derive(Template)]
#[template(path = "feed.html")]
pub struct Feed;

pub async fn build_template() -> Feed {
  Feed
}
