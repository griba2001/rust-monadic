// monad.rs

use std::iter::{IntoIterator, Iterator, FlatMap};
use std::collections::{LinkedList, VecDeque};

/// `Bind` as supertrait of `IntoIterator`
///
/// IntoIterator brings "into_iter().flat_map()" where its lazy result type FlatMap implements IntoIterator
///
/// This trait has been mostly an essay as it is not used because of the constraints and misrecognized instances explained below. 
///
/// Using into_iter() directly can be applied to more cases.
///
/// There are transitive implementation relations for some structures to be instances of IntoIterator: 
///
/// All iterators implement IntoIterator where into_iter() returns the self iterator structure
/// as [documented](https://doc.rust-lang.org/stable/core/iter/#for-loops-and-intoiterator) 
///
/// Some structures e.g. `Range` implement a supertrait of Iterator, so they are IntoIterator instances, 
/// but they are not recognized as instances of the defined Bind as supertrait of IntoIterator and its implementation for all IntoIterators, 
/// so the macro doesn't use the defined `bind()` but `into_iter().flatmap()`
///
/// Iterator and IntoIterator trait imports are [predefined](https://doc.rust-lang.org/std/prelude/index.html#prelude-contents)
///
/// Because into_iter() passes self by value, a `Sized` constraint (size known at compile time)
/// is required for Self in the use of `bind()`.
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
