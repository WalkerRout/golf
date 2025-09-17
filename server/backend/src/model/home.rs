use crate::build::home::Builder;

pub struct Home {
  pub name: String,
  pub age: u8,
}

impl From<Builder> for Home {
  fn from(builder: Builder) -> Self {
    Self {
      name: builder.name,
      age: builder.age,
    }
  }
}
