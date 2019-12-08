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
      M: 'a + Monad<Item=A> + FromIterator<A>,
{

  // This pure requires monad type annotated, better use lift( Vec::pure)
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
           M::bind( m, g).collect()
           })
       }
     }
     
     // initial_env returns the inner monad
     pub fn initial_env(self, e: E) -> M {
       (* self.run_reader_t) (e)
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

#[macro_export]
macro_rules! rdrt_mdo {
  (lift $nested_monad:expr                ) => [lift($nested_monad)];
  (guard $boolean:expr ; $($rest:tt)*) => [lift( if $boolean {vec![()]} else {vec![]}).bind( move |_| { rdrt_mdo!($($rest)*)} )];
  (_ <- $monad:expr ; $($rest:tt)* ) => [ReaderT::bind(($monad), move |_| { rdrt_mdo!($($rest)*)} )];
  ($v:ident <- lift $nested_monad:expr ; $($rest:tt)* ) => [ReaderT::bind( lift($nested_monad), move |$v| { rdrt_mdo!($($rest)*)} )];
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [ReaderT::bind(($monad), move |$v| { rdrt_mdo!($($rest)*)} )];
  ($monad:expr                            ) => [$monad];
}
