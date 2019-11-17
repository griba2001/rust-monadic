//!
//! Haskell style "monadic" macro 
//! where monad sources should be expressions of type instances of IntoIterator.
//!
//! Each monad expression is flat_mapped with the lambda expression having the monad result variable as argument and the rest as its body,
//! into a lazy FlatMap expression which can be collected into any instance of FromIterator..
//!
//! ```
//! # #[macro_use] extern crate monadic;
//!
//! # fn main() {
//!    let xs = monadic!{ 
//!        x <- 1..5;
//!        y <- 1..x;
//!        guard y < x;
//!        let z = y - 1;
//!        Option::pure((x,z)) 
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
        let xs = (1..5).collect::<Vec<i32>>();
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
