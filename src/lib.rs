//!
//! Haskell style "monadic" macro 
//! where monad sources should be expressions of type instances of IntoIterator.
//!
//! Each monad expression is flat_mapped with the lambda expression having the monad result variable as argument and the rest as its body,
//! into a lazy FlatMap expression that is also an instance of IntoIterator, and can be collected into any instance of FromIterator.
//!
//! To use `pure` to lift a value, a monad implementation must be used, beeing Option::pure(x) the least costly option.
//!
//! ```
//! # #[macro_use] extern crate monadic;
//! use monadic::{monadic, Monad};
//! use num::Integer;
//!
//! # fn main() {
//!    let xs = monadic!{ 
//!        x <- 1..5;
//!        y <- 1..x;
//!        guard x.is_odd();
//!        let z = y - 1;
//!        Vec::pure((x,z)) 
//!    }.collect::<Vec<(i32,i32)>>();
//!    
//!    println!("result: {:?}", xs); 
//! # }
//!
//!  // test:
//!
//!  fn it_works() {
//!        let xs = (1..5).collect::<Vec<i32>>();
//!        // expected
//!        let zs = (&xs).into_iter().filter(|&v| v < &4).map(|v| v*2).collect::<Vec<i32>>();
//!        // monadic
//!        let ys = monadic!{
//!           v <- &xs;
//!           guard v < &4;
//!           Option::pure( v * 2)
//!        }.collect::<Vec<i32>>();
//!        
//!        assert_eq!(ys, zs);
//!    }
//! ```

pub mod monad;
pub use monad::Monad; //reexporting Monad


/// converting monadic blocs of IntoIterator's as monads à la Haskell
///
/// You can use: 
/// * ```Option::pure( return_expresion)```  to return an expression value
/// * ```v <- monadic_expression```  to use the monad result
/// * ```_ <- monadic_expression```  to ignore the monad result
/// * ```let z = expression```       to combine monad results
/// * ```guard boolean_expression``` to filter results
///
/// it uses `into_iter().flat_map` instead of the defined `bind` for wider applicability since the latter requires the [Sized constraint](https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait)
#[macro_export]
macro_rules! monadic {
  (let $v:ident = $e:expr ; $($rest:tt)*) => [Some($e).into_iter().flat_map( move |$v| { monadic!($($rest)*)} )];
  (guard $pred:expr ; $($rest:tt)*) => [(if $pred {Some(())} else {None}).into_iter().flat_map( move |_| { monadic!($($rest)*)} )];
  (_ <- $monad:expr ; $($rest:tt)* ) => [($monad).into_iter().flat_map( move |_| { monadic!($($rest)*)} )];
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [($monad).into_iter().flat_map( move |$v| { monadic!($($rest)*)} )];
  ($monad:expr                            ) => [$monad];
}


#[cfg(test)]
mod tests {
    use std::vec::Vec; 
    use super::{monadic, Monad};
    #[test]
    fn it_works() {
        let xs = (1..6).collect::<Vec<i32>>();
        // expected
        let zs = (&xs).into_iter().filter(|&v| v < &4).map(|v| v*2).collect::<Vec<i32>>();
        // monadic
        let ys = monadic!{
           v <- &xs;
           guard v < &4;
           Option::pure( v * 2)
        }.collect::<Vec<i32>>();
        
        assert_eq!(ys, zs);
    }
}
