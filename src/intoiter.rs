//! definition of IntoIterator based monadic macro of Haskell style monadic action blocks

/// Macro which uses IntoIterator methods directly
///
/// You can use: 
/// * ```pure return_expresion```    to return an expression value
/// * ```monadic_expression```       to end with a monad expression
/// * ```v <- pure return_expresion```  to lift a rhs expression value with Some(x)
/// * ```v <- monadic_expression```  to use the monad result
/// * ```&v <- &monadic_expression```  to use an item by ref from a by ref monad
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
macro_rules! monadic {
  (pure $e:expr                           ) => [Some($e)];
  (let $v:ident = $e:expr ; $($rest:tt)*) => [Some($e).into_iter().flat_map( move |$v| { monadic!($($rest)*)} )];
  (guard $boolean:expr ; $($rest:tt)*) => [(if $boolean {Some(())} else {None}).into_iter().flat_map( move |_| { monadic!($($rest)*)} )];
  (_ <- $monad:expr ; $($rest:tt)* ) => [($monad).into_iter().flat_map( move |_| { monadic!($($rest)*)} )];
  (&$v:ident <- $monad:expr ; $($rest:tt)* ) => [($monad).into_iter().flat_map( move |&$v| { monadic!($($rest)*)} )];
  ($v:ident <- pure $e:expr ; $($rest:tt)* ) => [Some($e).into_iter().flat_map( move |$v| { monadic!($($rest)*)} )];
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [($monad).into_iter().flat_map( move |$v| { monadic!($($rest)*)} )];
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
           guard v < 4;
           pure v * 2
        }.collect::<Vec<i32>>();
        
        assert_eq!(ys, zs);
    }
}
