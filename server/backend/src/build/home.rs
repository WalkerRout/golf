use std::convert::Infallible;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::build::Build;

use crate::template::home::Home;

#[derive(Debug, Default)]
pub struct Builder {
  name: String,
  age: u8,
}

impl Builder {
  pub fn set_name(mut self, name: impl Into<String>) -> Self {
    self.name = name.into();
    self
  }

  pub fn set_age(mut self, age: impl Into<u8>) -> Self {
    self.age = age.into();
    self
  }
}

impl Build for Builder {
  type Target = Home;
  type Error = Infallible;

  fn build(self) -> Result<Self::Target, Self::Error> {
    Ok(Home {
      name: self.name,
      age: self.age,
    })
  }
}

pub async fn builder() -> Builder {
  Builder::default()
    .set_name("Walker Rout")
    .set_age(get_my_age())
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
