use askama::Template;

#[derive(Template)]
#[template(path = "feed.html")]
pub struct Feed;

pub fn build_template() -> Feed {
  Feed
}
