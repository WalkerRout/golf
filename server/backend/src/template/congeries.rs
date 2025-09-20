use askama::Template;

use crate::model::repo::Repo;

#[derive(Template, Clone)]
#[template(path = "congeries.html")]
pub struct Congeries {
  pub repositories: Vec<Repo>,
}
