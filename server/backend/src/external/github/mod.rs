use chrono::{DateTime, Utc};

use reqwest::{Client, Url};

use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("failed to fetch repos: {0}")]
  FetchFailed(#[from] reqwest::Error),

  #[error("")]
  UrlParseFailed(#[from] url::ParseError),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct DeserializedRepo {
  name: String,
  description: Option<String>,
  html_url: String,
  language: Option<String>,
  stargazers_count: u32,
  archived: bool,
  fork: bool,
  updated_at: DateTime<Utc>,
}

// cleaned up version for public use
#[derive(Clone)]
pub struct Repo {
  pub name: String,
  pub description: String,
  pub url: String,
}

impl From<DeserializedRepo> for Repo {
  fn from(gh_repo: DeserializedRepo) -> Self {
    Self {
      name: gh_repo.name,
      description: gh_repo
        .description
        .unwrap_or_else(|| "No description".to_string()),
      url: gh_repo.html_url,
    }
  }
}

pub async fn fetch_repositories() -> Result<Vec<Repo>, Error> {
  let url = {
    let mut parsed = Url::parse("https://api.github.com/users/WalkerRout/repos")?;
    parsed
      .query_pairs_mut()
      .append_pair("per_page", "200")
      .append_pair("type", "all")
      .append_pair("sort", "updated");
    parsed
  };
  let client = Client::builder().user_agent("golf-server").build()?;

  let gh_repos = client
    .get(url)
    .send()
    .await?
    .json::<Vec<DeserializedRepo>>()
    .await?;

  // we sort serverside...
  // gh_repos.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

  let repos: Vec<Repo> = gh_repos
    .into_iter()
    .filter(|r| !r.archived && !r.fork)
    .map(Repo::from)
    .collect();

  Ok(repos)
}
