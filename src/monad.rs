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
     
     // into_iter() passes self by value, so Self: Sized is required
     fn bind<U, F>(self, f: F) -> FlatMap<Self::IntoIter, U, F>
        where 
          F: Fn(Self::Item) -> U,
          U: IntoIterator,
          Self: Sized {
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

pub trait MonadPlus: Monad {
   fn mzero() -> Self;
   fn mplus(&self, _:&Self) -> Self;
}

impl<T:Clone> MonadPlus for Option<T> {

   fn mzero() -> Self {None}
   
   fn mplus(&self, b:&Self) -> Self {
      match (self, *(&b)) {
        (Some(_), Some(_)) => self.clone(),
        (None, _) => b.clone(),
        (_, None) => self.clone(),
      }
   }
}

impl<T:Clone> MonadPlus for Vec<T> {

   fn mzero() -> Self { Self::new() }
   
   fn mplus(&self, b:&Self) -> Self {
   
      let mut c = self.clone();
      c.append( &mut b.clone());
      c
   }
}

impl<T:Clone> MonadPlus for LinkedList<T> {

   fn mzero() -> Self { Self::new() }
   
   fn mplus(&self, b:&Self) -> Self {
   
      let mut c = self.clone();
      c.append( &mut b.clone());
      c
   }
}

impl<T:Clone> MonadPlus for VecDeque<T> {

   fn mzero() -> Self { Self::new() }
   
   fn mplus(&self, b:&Self) -> Self {
   
      let mut c = self.clone();
      c.append( &mut b.clone());
      c
   }
}


/// Macro based on Bind and Monad traits as supertraits of IntoIterator
///
/// You can use: 
/// * `pure return_expresion`    to return an expression value
/// * `monadic_expression`       to end with a monad expression
/// * `v <- pure return_expresion`  to lift a rhs expression value with Option::pure(x)
/// * `v <- monadic_expression`  to use the monad result
/// * `&v <- &monadic_expression`  to use a reference item result from a by reference monad
/// * `_ <- monadic_expression`  to ignore the monad result
/// * `let z = expression`       to combine monad results
/// * `guard boolean_expression` to filter results
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
    use crate::monad::{Bind, Monad, MonadPlus};
    use quickcheck::quickcheck;
    
    fn vec2option<T: Clone>( xs: Vec<T>) -> Option<T> {
        match &xs.len() {
          0 => None,
          1 => Some( xs[0].clone()),
          _ => panic!("vec2option: unexpectedly long vec"),
        }
    }

    #[test]
    fn prop_option_mplus_left_zero() {
    
        let z = Option::<i32>::mzero();
        let vs = (&z).bind( |&i: &i32| Some(i)).collect::<Vec<_>>();
        assert_eq!( vec2option(vs), z);
    }
    
    quickcheck!{
        fn prop_monad_comprehension_vs_iteration( xs: Vec<i32>) -> bool {
        
            // monadic
            let ys = mdo!{
                &v <- &xs;
                guard v < 4;
                pure v * 2
            }.collect::<Vec<i32>>();
            
            // as iterator
            let zs = (&xs).into_iter().filter(|&v| v < &4).map(|v| v*2).collect::<Vec<i32>>();
            
            ys == zs
        }
        
        fn prop_option_mplus_left_identity(a: Option<i32>) -> bool {
            Option::mzero().mplus(&a) == a
        }
        
        fn prop_option_mplus_right_identity(a: Option<i32>) -> bool {
            Option::mplus(&a, &Option::mzero()) == a
        }
        
        fn prop_option_mplus_associative( a: Option<i32>, 
                                          b: Option<i32>, 
                                          c: Option<i32>) -> bool {
           let ab = Option::mplus(&a, &b);
           let bc = Option::mplus(&b, &c);
           ab.mplus(&c) == (&a).mplus(&bc)
        }
        
        fn prop_option_mplus_left_catch(x: i32, 
                                        b: Option<i32>) -> bool {
           let a = Option::pure(x);
           (&a).mplus( &b) == a
        }
        
        fn prop_vec_mplus_left_identity(a: Vec<i32>) -> bool {
            Vec::mzero().mplus(&a) == a
        }
        
        fn prop_vec_mplus_right_identity(a: Vec<i32>) -> bool {
            Vec::mplus(&a, &Vec::mzero()) == a
        }
        
        fn prop_vec_mplus_associative( a: Vec<i32>, 
                                          b: Vec<i32>, 
                                          c: Vec<i32>) -> bool {
           let ab = Vec::mplus(&a, &b);
           let bc = Vec::mplus(&b, &c);
           ab.mplus(&c) == (&a).mplus(&bc)
        }
        
        fn prop_vec_mplus_left_distribution(a: Vec<i32>, 
                                          b: Vec<i32>) -> bool {
           let k = |&i: &i32| Vec::pure(i);                               
           let x = (&a).bind( k).collect::<Vec<_>>();
           let y = (&b).bind( k).collect::<Vec<_>>();
           let z = (&a).mplus(&b);
           (&z).bind( k).collect::<Vec<_>>() == (&x).mplus(&y)
        }
    }
}

