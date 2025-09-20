use askama::Template;

#[derive(Template, Clone)]
#[template(path = "home.html")]
pub struct Home {
  pub name: String,
  pub age: u8,
}
