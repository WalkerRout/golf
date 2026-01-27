use askama::Template;

use tracing::{info, warn};

use crate::external::github::{self, Repo};

#[derive(Template, Clone)]
#[template(path = "congeries.html")]
pub struct Congeries {
  pub repositories: Vec<Repo>,
}

pub async fn build_template() -> Congeries {
  match github::fetch_repositories().await {
    Ok(repositories) => {
      info!("congeries template cached");
      Congeries { repositories }
    }
    Err(e) => {
      warn!("failed to fetch github repos: {e}");
      Congeries {
        repositories: Vec::new(),
      }
    }
  }
}
