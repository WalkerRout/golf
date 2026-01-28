use askama::Template;

#[derive(Template, Clone)]
#[template(path = "about.html")]
pub struct About {
  pub name: String,
}

pub async fn build_template() -> About {
  About {
    name: "Walker Rout".to_string(),
  }
}
