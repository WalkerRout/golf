use serde::{Deserialize, Serialize};

use crate::build::repo::Builder;

#[derive(Clone, Serialize, Deserialize)]
pub struct Repo {
  pub name: String,
  pub description: String,
  pub url: String,
}

impl From<Builder> for Repo {
  fn from(builder: Builder) -> Self {
    Self {
      name: builder.name,
      description: builder.description,
      url: builder.url,
    }
  }
}
