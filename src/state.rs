// mod State

pub struct State<S, A> { 
  pub run_state: Box<dyn Fn(S) -> (A, S)>, 
}

impl<S: Clone + 'static, A: Copy + 'static> State<S, A> {

  pub fn pure(x: A) -> Self {
    State { run_state: Box::new( move |s: S| (x, s))}  // (s -> (a,s))
  }

  pub fn bind<B: 'static, F: 'static>(self, f: F) -> State<S, B> 
    where
      F: Fn(A) -> State<S, B>
  {
    State { run_state: Box::new( move |s: S| {
                  let (v, s1) = (*self.run_state) (s); // let (v,s') = runState self s
                  let g = f( v).run_state ;
                  (* g) (s1)             // runState (f v) s'
               })
          }     
  }

  pub fn initial_state(self, s: S) -> (A, S) {
       (*self.run_state) (s)
  }
  
}

pub fn get<S: Clone>() -> State<S, S> {
   State { run_state: Box::new( |s: S| (s.clone(), s))} 
}

pub fn put<S: Clone + 'static>( s: S) -> State<S, ()> {
   State { run_state: Box::new( move |_| ( (), s.clone()) )} 
}


#[macro_export]
macro_rules! stdo {
  (pure $e:expr                           ) => [State::pure($e)];
  (let $v:ident = $e:expr ; $($rest:tt)*) => [State::pure($e).bind( move |$v| { stdo!($($rest)*)} )];
  (_ <- $monad:expr ; $($rest:tt)* ) => [State::bind(($monad), move |_| { stdo!($($rest)*)} )];
  (&$v:ident <- $monad:expr ; $($rest:tt)* ) => [State::bind(($monad), move |&$v| { stdo!($($rest)*)} )];
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [State::bind(($monad), move |$v| { stdo!($($rest)*)} )];
  ($monad:expr                            ) => [$monad];
}

/*
// StateT definition fails to parse

use crate::monad::Monad;

pub struct StateT<S, M: Monad, A> { 
  pub run_state_t: Box<dyn Fn(S) -> M, where M::Item=(A,S) >, 
}
*/

