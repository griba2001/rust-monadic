//! definition of IntoIterator based monadic macro of Haskell style monadic action blocks

/// macro which uses IntoIterator as monad
///
/// You can use: 
/// * ```Some( return_expresion)```  to return an expression value
/// * ```v <- monadic_expression```  to use the monad result
/// * ```_ <- monadic_expression```  to ignore the monad result
/// * ```let z = expression```       to combine monad results
/// * ```guard boolean_expression``` to filter results
///
/// it uses `into_iter().flat_map` instead of the defined `bind` for wider applicability since the latter
/// requires the [Sized constraint](https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait)
///
/// There are transitive implementation relations for some structures to be instances of IntoIterator that only implement Iterator: 
///
/// All iterators implement IntoIterator where into_iter() returns the self iterator structure
/// as [documented](https://doc.rust-lang.org/stable/core/iter/#for-loops-and-intoiterator) 
///
/// Iterator and IntoIterator trait imports are [predefined](https://doc.rust-lang.org/std/prelude/index.html#prelude-contents)
///
#[macro_export]
macro_rules! monadic {
  (let $v:ident = $e:expr ; $($rest:tt)*) => [Some($e).into_iter().flat_map( move |$v| { monadic!($($rest)*)} )];
  (guard $boolean:expr ; $($rest:tt)*) => [(if $boolean {Some(())} else {None}).into_iter().flat_map( move |_| { monadic!($($rest)*)} )];
  (_ <- $monad:expr ; $($rest:tt)* ) => [($monad).into_iter().flat_map( move |_| { monadic!($($rest)*)} )];
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [($monad).into_iter().flat_map( move |$v| { monadic!($($rest)*)} )];
  (&$v:ident <- $monad:expr ; $($rest:tt)* ) => [($monad).into_iter().flat_map( move |$v| { monadic!($($rest)*)} )];
  ($monad:expr                            ) => [$monad];
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let xs = (1..6).collect::<Vec<i32>>();
        // as iterator
        let zs = (&xs).into_iter().filter(|&v| v < &4).map(|v| v*2).collect::<Vec<i32>>();
        // monadic
        let ys = monadic!{
           &v <- &xs;
           guard v < &4;
           Some( v * 2)
        }.collect::<Vec<i32>>();
        
        assert_eq!(ys, zs);
    }
}
