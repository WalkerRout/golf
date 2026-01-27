use std::time::{Duration, SystemTime, UNIX_EPOCH};

use askama::Template;

#[derive(Template, Clone)]
#[template(path = "home.html")]
pub struct Home {
  pub name: String,
  pub age: u8,
}

pub async fn build_template() -> Home {
  Home {
    name: "Walker Rout".to_string(),
    age: get_my_age(),
  }
}

fn get_my_age() -> u8 {
  // date -d "2004-06-05 UTC" +%s
  let reference = UNIX_EPOCH + Duration::from_secs(1086393600);
  let now = SystemTime::now();
  let years = now
    .duration_since(reference)
    .expect("time should not go backwards")
    .as_secs()
    / (365 * 24 * 60 * 60);
  (years & 0xFF) as u8
}
