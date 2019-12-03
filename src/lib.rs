//!
//! Haskell style block macros "mdo" and "monadic"
//! where monad sources should be expressions of type instances of IntoIterator.
//!
//! Each monad expression is flat_mapped with the lambda expression having the monad result variable as argument and the rest as its body,
//! into a lazy FlatMap expression that is also an instance of IntoIterator, and can be collected into any instance of FromIterator.
//!
//! The macro "mdo" (module `monad`) uses the traits Bind (IntoIterator supertrait) and Monad (Bind supertrait) defined there.
//!
//! The macro "monadic" (module `intoiter`) uses IntoIterator into_iter() without interpositions.
//!
//! To use `pure` to lift a value, it has to be typed by a monad implementation, 
//! being Option::pure(x) the least costly option, or just Some(x).
//!
//! There are transitive implementation relations for some structures to be instances of IntoIterator:
//!
//! All iterators implement IntoIterator where into_iter() returns the self iterator structure
//! as [documented](https://doc.rust-lang.org/stable/core/iter/#for-loops-and-intoiterator) 
//!
//! Iterator and IntoIterator trait imports are [predefined](https://doc.rust-lang.org/std/prelude/index.html#prelude-contents)
//!
//! ```
//! # #[macro_use] extern crate monadic;
//! use num::Integer;
//! use monadic::{mdo, monad::{Bind, Monad}};
//!
//! // alternatively you may use monadic::monadic; the macro without interpositions.
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
pub mod intoiter;
pub mod monoid;
pub mod reader;
pub mod writer;
pub mod util;
pub mod state;
pub mod mio;


