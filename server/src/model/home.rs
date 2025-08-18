use crate::model::Build;

#[derive(Debug, Default)]
pub struct Builder {
  pub name: String,
  pub age: u8,
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

impl Build for Builder {}
