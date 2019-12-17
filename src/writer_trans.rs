// writer_trans.rs 

#[allow(unused_imports)]
use crate::monad::{Bind, Monad, MZero, MPlus};
use crate::monoid::Monoid;
// use std::iter::FromIterator;
use std::collections::LinkedList;


#[derive(Clone)]  
pub struct WriterT<M, W = String>{ // M: Bind<Item=A> 
  run_writer_t: (M, W)
  }
 
impl<A, M, W> WriterT<M, W> 
  where 
       A: Clone, 
       M: Monad<Item = A> + Clone,  
       W: Monoid + Clone,
    {
    
   /// This function requires to type annotate the inner monad, better use `lift MonadInstance::pure(expr)`
   pub fn pure(x: A) -> Self
   {
     WriterT { run_writer_t: (M::pure( x.clone()), W::mempty())}  // Haskell (m a, mempty())
   }

   /// lift a monad
   pub fn lift(m: M) -> Self
   {
      WriterT {run_writer_t: (m, W::mempty())}
   }  
   
   /// the destination inner monad must implement crate::monad::MPlus
   pub fn bind<B: Clone, N: Clone, F>(self, f: F) -> WriterT<N, W>
     where 
       F: Fn(A) -> WriterT<N, W>,
       N: MPlus<Item = B>
   {
     let (m, w) = self.run_writer_t;
     let g = |a| LinkedList::pure(f(a).run_writer_t) ;
     let mut list = M::bind(m, g).collect::<LinkedList<(N, W)>>();
     
     if list.is_empty() {
        WriterT { run_writer_t: ( N::mzero(), w) }
     }
     else {
     
        let (mut n_out, mut w1) = (&mut list).pop_front().unwrap();
        let w_out = w.mappend(&mut w1);
        
        while let Some( (mut n_l, _)) = (&mut list).pop_front() {
            (&mut n_out).mplus( &mut n_l); 
        };
        WriterT { run_writer_t: (n_out, w_out)}
     }
   }

   /// `unwrap_pair` returns the inner monad and log
   pub fn unwrap_pair(self) -> (M, W) {
        self.run_writer_t
   }
    
    /// `unwrap` returns the inner monad
   pub fn unwrap(self) -> M {
        self.run_writer_t.0
   }

   /// `listen` pairs the result with the log
   pub fn listen(self) -> WriterT<Vec<(A, W)>, W> {
        let (m, w) = self.run_writer_t;
        let g = |a| vec!((a, (&w).clone()));
        let n = m.bind( g).collect::<Vec<(A, W)>>();
        WriterT{ run_writer_t: (n, w)}
   }
   
   /// `listens` pairs the result with a projection of the log
   pub fn listens<F, V>(self, f: F) -> WriterT<Vec<(A, V)>, W> 
      where
        F: Fn(W) -> V
   {
        let (m, w) = self.run_writer_t;
        let g = |a| vec!((a, f((&w).clone())));
        let n = m.bind( g).collect::<Vec<(A, V)>>();
        WriterT{ run_writer_t: (n, w)}
   }
   
   /// `censor` modifies the log
   pub fn censor<F: Fn(W) -> W>(self, f: F) -> Self 
   {
        let (m, w) = self.run_writer_t;
        WriterT{ run_writer_t: (m, f(w))}
   }
   
}

/*
/// lift a monad
pub fn lift<M: Bind, W: Monoid>(m: M) -> WriterT<M, W>
{
   WriterT {run_writer_t: (m, W::mempty())}
} 
*/

/// `censor` modifies the log
pub fn censor<M, W, F: Fn(W) -> W>(f: F, wrt: WriterT<M, W>) -> WriterT<M, W> 
{
        let (m, w) = wrt.run_writer_t;
        WriterT{ run_writer_t: (m, f(w))}
}

/// `listen` pairs the result with the log
pub fn listen<A, M: Bind<Item=A>, W: Clone>(wrt: WriterT<M, W>) -> WriterT<Vec<(A, W)>, W> {
        let (m, w) = wrt.run_writer_t;
        let g = |a| vec!((a, (&w).clone()));
        let n = m.bind( g).collect::<Vec<(A, W)>>();
        WriterT{ run_writer_t: (n, w)}
}

/// `listens` pairs the result with a projection of the log
pub fn listens<A, M: Bind<Item=A>, W: Clone, V, F: Fn(W) -> V>(wrt: WriterT<M, W>, f: F) -> WriterT<Vec<(A, V)>, W> {
        let (m, w) = wrt.run_writer_t;
        let g = |a| vec!((a, f((&w).clone())));
        let n = m.bind( g).collect::<Vec<(A, V)>>();
        WriterT{ run_writer_t: (n, w)}
}

/// `tell` sets the log value
pub fn tell<W: Clone>(s: W) -> WriterT<Vec<()>, W> {
        WriterT{ run_writer_t: (vec!(()), s.clone())}
    }

pub fn tell_str(s: &str) -> WriterT<Vec<()>, String> {
        WriterT{ run_writer_t: (vec!(()), String::from( s))}
    }

pub fn tell_array<T: Clone>(v: &[T]) -> WriterT<Vec<()>, Vec<T>> {
        WriterT{ run_writer_t: (vec!(()), Vec::from( v))}
    }
    
/// WriterT monad transformer macro for a `WriterT<M, W> {run_writer_t: (M, W)} where M: Monad, W: Monoid;`
/// It uses the type alias Log in type annotations
#[macro_export]
macro_rules! wrt_mdo {
  (lift $nested_monad:expr                ) => [WriterT::lift($nested_monad)];
  (pure $e:expr                ) => [WriterT::lift(vec!($e))];
  (guard $boolean:expr ; $($rest:tt)*) => [WriterT::lift( if $boolean {vec![()]} else {vec![]}).bind( move |_| { wrt_mdo!($($rest)*)} )];
  (_ <- tell_str $str:literal ; $($rest:tt)* ) => [(tell_str($str) as WriterT<Vec<_>, Log>).bind( 
                                                      move |_| { wrt_mdo!($($rest)*)} )];
  (_ <- tell_array $ar:expr ; $($rest:tt)* ) => [(tell_array($ar) as WriterT<Vec<_>, Log>).bind( 
                                                      move |_| { wrt_mdo!($($rest)*)} )];
  (_ <- tell_vec $e:expr ; $($rest:tt)* ) => [(tell($e) as WriterT<Vec<_>, Log>).bind( 
                                                      move |_| { wrt_mdo!($($rest)*)} )];
  (_ <- tell_string $e:expr ; $($rest:tt)* ) => [(tell($e) as WriterT<Vec<_>, Log>).bind( 
                                                      move |_| { wrt_mdo!($($rest)*)} )];
  (_ <- $monad:expr ; $($rest:tt)* ) => [WriterT::bind(($monad), move |_| { wrt_mdo!($($rest)*)} )];
  (let $v:ident = $e:expr ; $($rest:tt)* ) => [WriterT::lift(vec!($e)).bind( move |$v| { wrt_mdo!($($rest)*)} )];
  ($v:ident <- lift $nested_monad:expr ; $($rest:tt)* ) => [WriterT::bind( WriterT::lift($nested_monad), move |$v| { wrt_mdo!($($rest)*)} )];
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [WriterT::bind(($monad), move |$v| { wrt_mdo!($($rest)*)} )];
  ($monad:expr                            ) => [$monad];
}

