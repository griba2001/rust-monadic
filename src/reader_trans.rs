// reader_trans

use crate::monad::{Monad};
use std::iter::FromIterator;


pub struct ReaderT<'a, E, M> {                 // M: Monad + FromIterator
  pub run_reader_t: Box< dyn 'a + Fn(E) -> M>, 
}

impl<'a, A, E, M> ReaderT<'a, E, M> 
    where
      E: 'a + Clone, 
      A: 'a + Clone,
      M: 'a + Clone + Monad<Item=A> + FromIterator<A>, 
{

  /// This function requires to type annotate the inner monad, better use lift( MonadInstance::pure)
  pub fn pure(x: A) -> Self {
    ReaderT { run_reader_t: Box::new( move |_| M::pure( x.clone() ))}  // (e -> a)
  }
  
  pub fn bind<B, N, F>(self, f: F) -> ReaderT<'a, E, N>
        where 
          F: 'a + Fn(A) -> ReaderT<'a, E, N>,
          B: 'a,
          N: 'a + Monad<Item=B> + FromIterator<B>,
     {
       ReaderT { run_reader_t: 
           Box::new( move |e: E| { 
           let m = (* self.run_reader_t)( e.clone());
           let g = |a| (* f(a).run_reader_t)( e.clone());
           M::bind( m, g).collect::<N>()                        
           })
       }
     }
     
     // applying initial_env() to (e -> m a) returns the inner monad structure
     pub fn initial_env(self, e: E) -> M {
       (* self.run_reader_t) (e)
     }

     /// lift a monad
     pub fn lift(m: M) -> ReaderT<'a, E, M> {
        ReaderT { run_reader_t: Box::new( move |_| m.clone() )}
     }
     
     /// lift from iterator
     pub fn lift_iter<I>( it: I) -> ReaderT<'a, E, M> 
       where 
         I: 'a + Iterator<Item=A> + Clone,
     {
        ReaderT { run_reader_t: Box::new( move |_| it.clone().collect::<M>() )}
     }
}

pub fn ask<'a, E: Clone, M: Monad<Item=E>>() -> ReaderT<'a, E, M> {

  ReaderT { run_reader_t: Box::new(|e: E| M::pure( e.clone()))}
}


pub fn local<'a, E, M, F>(f: F, rdr: ReaderT<'a, E, M>) -> ReaderT<'a, E, M>
     where
       F: 'a + Fn(E) -> E,
       E: 'a, 
       M: 'a,
  {

    ReaderT { run_reader_t: 
           Box::new(move |e: E| { (*rdr.run_reader_t) (f(e)) })
        }
  }

pub fn lift<'a, E: 'a, M: 'a + Clone>(m: M) -> ReaderT<'a, E, M> {
     ReaderT { run_reader_t: Box::new( move |_| m.clone() )}
}

/// macro for a `ReaderT<'a, E, M>` monad transformer with a boxed `(env -> m a) where M: Monad + FromIterator`. 
/// It uses the type alias Env in type annotations
#[macro_export]
macro_rules! rdrt_mdo {
  (lift $last_nested_monad:expr                ) => [ReaderT::lift($last_nested_monad)];
  
  (pure $last_expr:expr                ) => [ReaderT::lift(vec!($last_expr))];
  
  (guard $boolean:expr ; $($rest:tt)*) => [ReaderT::lift( if $boolean {vec![()]} else {vec![]}).bind( move |_| { rdrt_mdo!($($rest)*)} )];
  
  (let $v:ident = $e:expr ; $($rest:tt)*) => [ReaderT::lift(vec![$e]).bind( move |$v| { rdrt_mdo!($($rest)*)} )];
  
  (_ <- $monad:expr ; $($rest:tt)* ) => [ReaderT::bind(($monad), move |_| { rdrt_mdo!($($rest)*)} )];
  
  ($v:ident <- ask() ; $($rest:tt)* ) => [( ask() as ReaderT<'_, Env, Vec<Env>>).bind( 
                                                         move |$v| { rdrt_mdo!($($rest)*)}) ];
                                                         
  ($v:ident <- lift_iter $iterator:expr ; $($rest:tt)* ) => [ReaderT::<'_, Env, Vec<_>>::lift_iter($iterator).bind( move |$v| { rdrt_mdo!($($rest)*)} )];
  
  (& $v:ident <- lift $nested_monad:expr ; $($rest:tt)* ) => [ReaderT::lift($nested_monad).bind( move |& $v| { rdrt_mdo!($($rest)*)} )];
  
  ($v:ident <- lift $nested_monad:expr ; $($rest:tt)* ) => [ReaderT::lift($nested_monad).bind( move |$v| { rdrt_mdo!($($rest)*)} )];
  
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [ReaderT::bind(($monad), move |$v| { rdrt_mdo!($($rest)*)} )];
  
  ($monad:expr                            ) => [$monad];
}
