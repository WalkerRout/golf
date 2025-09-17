use askama::Template;

use crate::model::congeries;
use crate::model::repo;

#[derive(Template, Clone)]
#[template(path = "congeries.html")]
pub struct Congeries {
  pub repositories: Vec<repo::Repo>,
}

impl<B> From<B> for Congeries
where
  B: Into<congeries::Congeries>,
{
  fn from(builder: B) -> Self {
    let congeries = builder.into();
    Self {
      repositories: congeries.repositories,
    }
  }
}
