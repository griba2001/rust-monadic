//!
//! Haskell style block macros "mdo"
//! where monad sources should be expressions of type instances of IntoIterator.
//!
//! Each monad expression is flat_mapped (`into_iter().flat_map( lambda)`) 
//! with the lambda expression having the monad result variable as argument and the rest as its body,
//! into a lazy FlatMap expression that is also an instance of IntoIterator, and can be collected into any instance of FromIterator.
//!
//! To use "mdo" (module `monad`) you must import the traits Bind and Monad defined there.
//!
//! There are transitive implementation relations for some structures to be instances of IntoIterator:
//!
//! All iterators implement IntoIterator where into_iter() returns the self iterator structure
//! as [documented](https://doc.rust-lang.org/stable/core/iter/#for-loops-and-intoiterator) 
//!
//! Iterator and IntoIterator trait imports are [predefined](https://doc.rust-lang.org/std/prelude/index.html#prelude-contents)
//!
//! There are also Reader, Writer and State monads in their respective modules with their own macros.
//!
//! ```no_run
//! # #[macro_use] extern crate monadic;
//! use num::Integer;
//! use monadic::{mdo, monad::{Bind, Monad}};
//!
//! # fn main() {
//!    // available in examples/comprehension.rs
//!
//!    let xs = mdo!{ 
//!
//!            x <- 1..7;
//!            y <- 1..x;
//!            guard (&y).is_odd() ;
//!            let z = match x.is_even() { 
//!                        true => &y + 1,
//!                        _ => &y - 1,
//!                    };
//!            pure (x, z)
//!            
//!    }.collect::<Vec<(i32,i32)>>();
//!    
//!    println!("result: {:?}", xs); 
//!
//!    // now with container and item references, available in examples/comprehension2.rs
//!
//!    let ys = mdo!{ 
//!    
//!        &x <- &vec![1,2,3,4];  // with item refs (&x) in the lambda argument position
//!        guard x.is_odd() ;
//!        let z = x + 1 ;
//!        pure (x, z)
//!        
//!    }.collect::<Vec<(i32,i32)>>();
//!    
//!    println!("result: {:?}", ys); 
//! # }
//! ```

pub mod monad;
pub mod mio;

#[cfg(feature="reader")]
pub mod reader;

#[cfg(feature="reader_trans")]
pub mod reader_trans;

#[cfg(feature="writer")]
pub mod writer;

#[cfg(feature="writer_trans")]
pub mod writer_trans;


#[cfg(feature="state")]
pub mod state;

#[cfg(feature="state_trans")]
pub mod state_trans;

#[cfg(any(feature="writer", feature="writer_trans"))]
pub mod util;

#[cfg(any(feature="writer", feature="writer_trans"))]
pub mod monoid;
