use std::convert::Infallible;

use tracing::{info, warn};

use crate::build::Build;
use crate::model::repo::Repo;
use crate::service::github;
use crate::template::congeries::Congeries;

#[derive(Default)]
pub struct Builder {
  repositories: Vec<Repo>,
}

impl Builder {
  #[allow(dead_code)]
  pub fn add(mut self, repo: impl Into<Repo>) -> Self {
    self.repositories.push(repo.into());
    self
  }

  pub fn add_many<R>(mut self, repos: impl IntoIterator<Item = R>) -> Self
  where
    R: Into<Repo>,
  {
    self.repositories.extend(repos.into_iter().map(Into::into));
    self
  }
}

impl Build for Builder {
  type Target = Congeries;
  type Error = Infallible;

  fn build(self) -> Result<Self::Target, Self::Error> {
    Ok(Congeries {
      repositories: self.repositories,
    })
  }
}

pub async fn builder() -> Builder {
  match github::fetch_repositories().await {
    Ok(repositories) => {
      info!("congeries template cached");
      Builder::default().add_many(repositories)
    }
    Err(e) => {
      warn!("failed to fetch github repos: {e}");
      Builder::default()
    }
  }
}
