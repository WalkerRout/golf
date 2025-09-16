use crate::build::Build;

#[derive(Debug, Default)]
pub struct Builder {
  pub name: String,
  pub description: String,
  pub url: String,
}

impl Builder {
  pub fn set_name(mut self, name: impl Into<String>) -> Self {
    self.name = name.into();
    self
  }

  pub fn set_description(mut self, description: impl Into<String>) -> Self {
    self.description = description.into();
    self
  }

  pub fn set_url(mut self, url: impl Into<String>) -> Self {
    self.url = url.into();
    self
  }
}

impl Build for Builder {}
