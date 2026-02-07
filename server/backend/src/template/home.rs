use askama::Template;

#[derive(Template, Clone)]
#[template(path = "home.html")]
pub struct Home {
  pub name: String,
}

pub async fn build_template() -> Home {
  Home {
    name: "Walker Rout".to_string(),
  }
}
