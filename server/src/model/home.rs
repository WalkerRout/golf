use crate::model::Build;

#[derive(Debug, Default)]
pub struct Builder {
  pub name: String,
}

impl Builder {
  pub fn set_name(mut self, name: impl Into<String>) -> Self {
    self.name = name.into();
    self
  }
}

impl Build for Builder {}
