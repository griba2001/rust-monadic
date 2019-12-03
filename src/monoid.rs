//! Semigroup and Monoid to use with Writer

pub trait Semigroup {

   fn mappend(self, other: &mut Self) -> Self;
}

pub trait Monoid: Semigroup {
   fn mempty() -> Self;
}

//--------------------------------------------

impl Semigroup for String {

  fn mappend( mut self, other: &mut Self) -> Self {
   self.push_str( other);
   self
  }
}

impl Monoid for String {
  fn mempty() -> Self { Self::new()}
}

//--------------------------------------------

use std::clone::Clone;

impl<T: Clone> Semigroup for Vec<T> {

  fn mappend( mut self, other: &mut Self) -> Self {
   self.append( other);
   self
  }
}

impl<T: Clone> Monoid for Vec<T> {
  fn mempty() -> Self { Self::new()}
}
