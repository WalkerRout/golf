use askama::Template;

use crate::model::home::*;

#[derive(Template, Clone)]
#[template(path = "home.html")]
pub struct Home {
  pub name: String,
  pub age: u8,
}

impl From<Builder> for Home {
  fn from(builder: Builder) -> Self {
    Self {
      name: builder.name,
      age: builder.age,
    }
  }
}
