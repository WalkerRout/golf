pub mod congeries;
pub mod home;

pub trait Build {
  type Target;
  type Error;

  fn build(self) -> Result<Self::Target, Self::Error>;
}
