use serde::{Deserialize, Serialize};

use crate::build::congeries::Builder;

use crate::model::repo::Repo;

#[derive(Serialize, Deserialize)]
pub struct Congeries {
  pub repositories: Vec<Repo>,
}

impl From<Builder> for Congeries {
  fn from(builder: Builder) -> Self {
    Self {
      repositories: builder.repositories,
    }
  }
}
