//!
//! haskell style "monadic" macro 
//! where monad sources should be collections implementing IntoIterator 
//! consuming its elements to build the result
//! and flat_mapping at each instruction
//!
//! You can use 
//! ```Some( returnValue)```       a monadic return
//! ```v <- monadic_expression```  using the monad result
//! ```_ <- monadic_expression```  ignoring the monad result
//! ```let z = expression```       combinig monad results
//! ```guard boolean_expression``` to filter results
//!


/// ```
/// # #[macro_use] extern crate monadic;
///
/// # fn main() {
///    let xs = monadic!{ 
///        x <- 1..5;
///        y <- 1..=x;
///        guard y < x;
///        let z = y - 1;
///        Some((x,z)) 
///    }.collect::<Vec<(i32,i32)>>();
///    
///    println!("result: {:?}", xs); 
/// # }
/// ```
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
    use super::monadic;
    #[test]
    fn it_works() {
        let xs = (1..5).collect::<Vec<i32>>();
        // expected
        let zs = (&xs).into_iter().filter(|&v| v < &4).map(|v| v*2).collect::<Vec<i32>>();
        // monadic
        let ys = monadic!{
           v <- &xs;
           guard v < &4;
           Some( v * 2)
        }.collect::<Vec<i32>>();
        
    
        assert_eq!(ys, zs);
    }
}
