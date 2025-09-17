use askama::Template;

use crate::model::home;

#[derive(Template, Clone)]
#[template(path = "home.html")]
pub struct Home {
  pub name: String,
  pub age: u8,
}

impl<B> From<B> for Home
where
  B: Into<home::Home>,
{
  fn from(builder: B) -> Self {
    let home = builder.into();
    Self {
      name: home.name,
      age: home.age,
    }
  }
}
