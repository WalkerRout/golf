pub mod cv;
pub mod home;

pub trait Build: Sized {
  fn build<T>(self) -> T
  where
    T: From<Self>,
  {
    T::from(self)
  }
}
