pub mod congeries;
pub mod home;
pub mod repo;

pub trait Build: Sized {
  fn build<T>(self) -> T
  where
    T: From<Self>,
  {
    T::from(self)
  }
}
