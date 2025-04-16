pub mod cv;

pub trait Build: Sized {
  fn build<T>(self) -> T
  where
    T: From<Self>,
  {
    T::from(self)
  }
}
