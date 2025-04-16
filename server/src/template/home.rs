use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home {
  pub name: String,
}
