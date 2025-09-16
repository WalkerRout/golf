use crate::build::repo;
use crate::build::Build;

use crate::model::repo::Repo;

#[derive(Default)]
pub struct Builder {
  pub repositories: Vec<Repo>,
}

impl Builder {
  pub fn add(mut self, repo: impl Into<Repo>) -> Self {
    self.repositories.push(repo.into());
    self
  }
}

impl Build for Builder {}

// fallback builder with hardcoded repos
pub fn builder() -> Builder {
  Builder::default()
    .add(
      repo::Builder::default()
        .set_name("golf")
        .set_description("A minimal static site generator written in Rust")
        .set_url("https://github.com/walkerrout/golf"),
    )
}
