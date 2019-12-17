// state_trans.rs 

use crate::monad::Monad;
use std::iter::FromIterator;

pub struct StateT<'a, S, M, A> 
where
  M: 'a + Monad<Item=(A, S)>,
{ 
  pub run_state_t: Box<dyn 'a + Fn(S) -> M>, 
}

impl<'a, A, S, M> StateT<'a, S, M, A> 
  where 
       A: 'a + Clone, 
       S: 'a + Clone,
       M: 'a + Monad<Item =(A, S)> + FromIterator<(A,S)> // FromIterator required in lift
    {
  /// This function requires type annotation of the inner monad
  pub fn pure(x: A) -> Self
  {
    StateT { run_state_t: Box::new( move |s: S| M::pure(( x.clone(), s)))}  // (s -> return (a,s))
  }
  
  
  pub fn lift<N>(n: N) -> Self
    where
      N: 'a + Clone + Monad<Item=A>,
  {
    StateT { run_state_t: Box::new( 
                        // pair each element with St
                        move |s| n.clone().into_iter().map( | a| (a, s.clone()) 
                                                          ).collect::<M>()
                        )}
  }
  
  pub fn lift_iter<I>(it: I) -> Self
    where
      I: 'a + Clone + Iterator<Item=A>,
  {
    StateT { run_state_t: Box::new( 
                        // pair each element with St
                        move |s| it.clone().map( | a| (a, s.clone()) 
                                                          ).collect::<M>()
                        )}
  }
  
  /// FromIterator is required to convert the inner monad bind output FlatMap struct to the Monad instance
  pub fn bind<N, B, F: 'a>(self, f: F) -> StateT<'a, S, N, B>
     where 
       F: 'a + Copy + Fn(A) -> StateT<'a, S, N, B>,
       N: 'a + Monad<Item=(B, S)> + FromIterator<(B, S)>,
       B: 'a,
  {
    StateT { run_state_t: Box::new( move |s: S| {
                  let m = (*self.run_state_t) (s); // the monad
                  let g = move |(v, s1)| (* f( v).run_state_t) (s1);
                  M::bind( m, g).collect::<N>()
                  })}
                 
   }
   
   
    /// it returns the inner monad structure use collect::<Vec<(_,_)>>()
    pub fn initial_state(self, s: S) -> M {
        (*self.run_state_t) (s)
    }
}

pub fn get<'a, S>() -> StateT<'a, S, Vec<(S, S)>, S> 
  where
    S: 'a + Clone, 
{
   StateT { run_state_t: Box::new( |s: S| {let p = (s.clone(), s); Vec::pure(p)}
                                 )} 
}

pub fn put<'a, S>( s: S) -> StateT<'a, S, Vec<((), S)>, ()> 
  where
    S: 'a + Clone, 
{
   StateT { run_state_t: Box::new( move |_| {let p = ((), s.clone()); Vec::pure(p)} 
                                 )} 
}

/* unused
pub fn lift<'a, S, A, M, N>(n: N) -> StateT<'a, S, M, A> 
  where
    A: 'a,
    S: 'a + Clone,
    N: 'a + Clone + Monad<Item=A>,
    M: 'a + Clone + Monad<Item=(A, S)> + FromIterator<(A, S)>, 
{
    StateT { run_state_t: Box::new( 
                        // pair each element with St
                        move |s| n.clone().into_iter().map( | a| (a, s.clone()) 
                                                          ).collect::<M>()
                        )}
}
*/

/// StateT transformer macro for a `StateT<'a, S, M, A> {run_state_t: Box(a -> m (a, s))} where S = St, M: Monad`;
/// It uses the type alias St in type annotations.
#[macro_export]
macro_rules! stt_mdo {
  (pure $e:expr)                       => [StateT::<'_, St, Vec<_>, _>::pure($e)];
  (lift $nested_monad:expr)            => [StateT::<'_, St, Vec<_>, _>::lift($nested_monad)];
  (guard $boolean:expr ; $($rest:tt)*) => [StateT::<'_, St, Vec<_>, _>::lift(if $boolean {vec![()]} else {vec![]}).bind( move |_| { stt_mdo!($($rest)*)} )];
  (_ <- $monad:expr ; $($rest:tt)* ) => [StateT::bind(($monad), move |_| { stt_mdo!($($rest)*)} )];
  ($v:ident <- lift_iter $it:expr ; $($rest:tt)* ) => [StateT::<'_, St, Vec<_>, _>::lift_iter($it).bind( move |$v| { stt_mdo!($($rest)*)} )];
  ($v:ident <- lift $nested_monad:expr ; $($rest:tt)* ) => [StateT::<'_, St, Vec<_>, _>::lift($nested_monad).bind( move |$v| { stt_mdo!($($rest)*)} )];
  (let $v:ident = $e:expr ; $($rest:tt)* ) => [StateT::bind(StateT::<'_, St, Vec<_>, _>::pure($e), move |$v| { stt_mdo!($($rest)*)} )];
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [StateT::bind(($monad), move |$v| { stt_mdo!($($rest)*)} )];
  ($monad:expr                            ) => [$monad];
}

