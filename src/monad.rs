//! definition of Bind and Monad traits based monadic macro 

use std::iter::{IntoIterator, Iterator, FlatMap};
use std::collections::{LinkedList, VecDeque};

/// `Bind` as supertrait of `IntoIterator`
///
/// IntoIterator brings `into_iter().flat_map()` where its lazy result type FlatMap implements IntoIterator.
/// It has been wrapped as `bind()` in trait Bind.
///
/// There are transitive implementation relations for some structures that does not implement IntoIterator to be instances of IntoIterator: 
///
/// All iterators implement IntoIterator where into_iter() returns the self iterator structure
/// as [documented](https://doc.rust-lang.org/stable/core/iter/#for-loops-and-intoiterator) 
///
/// Iterator and IntoIterator trait imports are [predefined](https://doc.rust-lang.org/std/prelude/index.html#prelude-contents)
///
/// Because into_iter() passes self by value, a `Sized` constraint (size known at compile time)
/// is required for Self in the use of `bind()`.
pub trait Bind: IntoIterator { 
     
     // // into_iter() passes self by value, so Self: Sized is required
     fn bind<U, F>(self, f: F) -> FlatMap<Self::IntoIter, U, F>
        where 
          F: Fn(Self::Item) -> U,
          U: IntoIterator,
          Self: Sized;
   }
   
   
impl<R> Bind for R where R: IntoIterator {

     fn bind<U, F>(self, f: F) -> FlatMap<Self::IntoIter, U, F>
        where 
          F: Fn(Self::Item) -> U,
          U: IntoIterator,
          Self: Sized,
     {
        self.into_iter().flat_map( f) // into_iter() passes self by value, so Self: Sized is required
     }
}  


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

/// Macro based on Bind and Monad traits as supertraits of IntoIterator
///
/// You can use: 
/// * ```pure return_expresion```    to return an expression value
/// * ```monadic_expression```       to end with a monad expression
/// * ```v <- pure return_expresion```  to lift a rhs expression value with Option::pure(x)
/// * ```v <- monadic_expression```  to use the monad result
/// * ```&v <- &monadic_expression```  to use a reference item result from a by reference monad
/// * ```_ <- monadic_expression```  to ignore the monad result
/// * ```let z = expression```       to combine monad results
/// * ```guard boolean_expression``` to filter results
///
/// There are transitive implementation relations for some structures to be instances of IntoIterator that only implement Iterator: 
///
/// All iterators implement IntoIterator where into_iter() returns the self iterator structure
/// as [documented](https://doc.rust-lang.org/stable/core/iter/#for-loops-and-intoiterator) 
///
/// Iterator and IntoIterator trait imports are [predefined](https://doc.rust-lang.org/std/prelude/index.html#prelude-contents)
///
#[macro_export]
macro_rules! mdo {
  (pure $e:expr                           ) => [Option::pure($e)];
  (let $v:ident = $e:expr ; $($rest:tt)*) => [Option::pure($e).bind( move |$v| { mdo!($($rest)*)} )];
  (guard $boolean:expr ; $($rest:tt)*) => [(if $boolean {Some(())} else {None}).bind( move |_| { mdo!($($rest)*)} )];
  (_ <- $monad:expr ; $($rest:tt)* ) => [($monad).bind( move |_| { mdo!($($rest)*)} )];
  (&$v:ident <- $monad:expr ; $($rest:tt)* ) => [($monad).bind( move |&$v| { mdo!($($rest)*)} )];
  ($v:ident <- pure $e:expr ; $($rest:tt)* ) => [Option::pure($e).bind( move |$v| { mdo!($($rest)*)} )];
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [($monad).bind( move |$v| { mdo!($($rest)*)} )];
  ($monad:expr                            ) => [$monad];
}

#[cfg(test)]
mod tests {
    use crate::monad::{Bind, Monad};
    #[test]
    fn it_works() {
        let xs = (1..6).collect::<Vec<i32>>();
        // as iterator
        let zs = (&xs).into_iter().filter(|&v| v < &4).map(|v| v*2).collect::<Vec<i32>>();
        // monadic
        let ys = mdo!{
           &v <- &xs;
           guard v < 4;
           pure v * 2
        }.collect::<Vec<i32>>();
        
        assert_eq!(ys, zs);
    }
}

