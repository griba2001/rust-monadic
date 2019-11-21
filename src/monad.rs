// monad.rs

use std::iter::{IntoIterator, Iterator, FlatMap};
use std::collections::{LinkedList, VecDeque};

/// `Bind` as supertrait of `IntoIterator`
///
/// IntoIterator brings "into_iter().flat_map()" where its lazy result type FlatMap implements IntoIterator
///
/// This trait has been mostly an essay as it is not used. 
///
/// Using into_iter() directly can be applied to more cases.
///
/// There are transitive implementation relations for some structures to be instances of IntoIterator: 
///
/// Some structures e.g. `Range` implement a supertrait of Iterator, which in turn implements IntoIterator 
/// `impl<I: Iterator> IntoIterator for I` ∀ I:Iterator,
/// as [documented](https://doc.rust-lang.org/stable/core/iter/#for-loops-and-intoiterator) 
/// so bringing `std::iter::Iterator` into scope could be useful
///
/// Because into_iter() passes self by value a `Sized` constraint (size known at compile time)
/// is imposed in this supertrait.
pub trait Bind: IntoIterator + Sized { 
     
     fn bind<U, F>(self, f: F) -> FlatMap<Self::IntoIter, U, F>
        where 
          F: Fn(Self::Item) -> U,
          U: IntoIterator,
     {
        
        self.into_iter().flat_map( f)
     }
   }
   
impl<R> Bind for R where R: IntoIterator {}  


pub trait Monad: Bind { 

     fn pure(x: Self::Item) -> Self;
   }

impl<T> Monad for Option<T> {
   fn pure(x: T) -> Self {
      Some(x)
   }
}

impl<T,E> Monad for Result<T,E>{
   fn pure(x: T) -> Self {
      Ok(x)
   }
}

impl<T> Monad for Vec<T>{
   fn pure(x: T) -> Self {
      let mut v = Self::new();
      v.push(x);
      v
   }
}

impl<T> Monad for LinkedList<T>{
   fn pure(x: T) -> Self {
      let mut v = Self::new();
      v.push_front(x);
      v
   }
}

impl<T> Monad for VecDeque<T>{
   fn pure(x: T) -> Self {
      let mut v = Self::new();
      v.push_front(x);
      v
   }
}
