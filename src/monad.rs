// monad.rs

use std::iter::{IntoIterator};
use std::collections::{LinkedList, VecDeque};

/// Monad<T> as supertrait of IntoIterator<Item=T>
///
/// IntoIterator brings "into_iter().flat_map()" where its lazy result type FlatMap implements IntoIterator
///
pub trait Monad<T>: IntoIterator<Item=T> { 

     fn pure(x: T) -> Self;
   }

impl<T> Monad<T> for Option<T>{
   fn pure(x: T) -> Self {
      Some(x)
   }
}

impl<T,E> Monad<T> for Result<T,E>{
   fn pure(x: T) -> Self {
      Ok(x)
   }
}

impl<T> Monad<T> for Vec<T>{
   fn pure(x: T) -> Self {
      let mut v = Self::new();
      v.push(x);
      v
   }
}

impl<T> Monad<T> for LinkedList<T>{
   fn pure(x: T) -> Self {
      let mut v = Self::new();
      v.push_front(x);
      v
   }
}

impl<T> Monad<T> for VecDeque<T>{
   fn pure(x: T) -> Self {
      let mut v = Self::new();
      v.push_front(x);
      v
   }
}