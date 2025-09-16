use chrono::{DateTime, Utc};

use reqwest::Client;

use serde::Deserialize;

use crate::model::congeries::Congeries;
use crate::model::repo::Repo;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Failed to fetch repos: {0}")]
  FetchFailed(#[from] reqwest::Error),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GitHubRepo {
  name: String,
  description: Option<String>,
  html_url: String,
  language: Option<String>,
  stargazers_count: u32,
  archived: bool,
  fork: bool,
  updated_at: DateTime<Utc>,
}

impl From<GitHubRepo> for Repo {
  fn from(gh_repo: GitHubRepo) -> Self {
    Self {
      name: gh_repo.name,
      description: gh_repo
        .description
        .unwrap_or_else(|| "No description".to_string()),
      url: gh_repo.html_url,
    }
  }
}

pub async fn fetch_repositories() -> Result<Congeries, Error> {
  let url = "https://api.github.com/users/WalkerRout/repos";
  let client = Client::builder().user_agent("golf-server").build()?;

  let mut gh_repos = client
    .get(url)
    .send()
    .await?
    .json::<Vec<GitHubRepo>>()
    .await?;

  gh_repos.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

  let repos: Vec<Repo> = gh_repos
    .into_iter()
    .filter(|r| !r.archived && !r.fork)
    .map(Repo::from)
    .collect();

  Ok(Congeries {
    repositories: repos,
  })
}