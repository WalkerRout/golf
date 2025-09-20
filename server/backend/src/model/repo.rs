use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Repo {
  pub name: String,
  pub description: String,
  pub url: String,
}
