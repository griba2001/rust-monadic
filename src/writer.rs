// mod writer
use crate::monoid::{Monoid};

//---------------------------------------------

#[derive(Clone)]  
pub struct Writer<A, W = String>{ 
  run_writer: (A, W)
  }

#[derive(Clone)]  
pub struct WriterIterator<A> {
  traversed: bool,
  value: A,
}


impl<A: Clone> Iterator for WriterIterator<A> {
   type Item = A;
   
   fn next(&mut self) -> Option<A> {
     if self.traversed {None} 
     else {self.traversed = true;
           Some( self.value.clone())}
   }
}

impl<A: Clone, W> IntoIterator for  Writer<A, W> {
  type Item = A;
  type IntoIter = WriterIterator<A>; 
  
  fn into_iter(self) -> Self::IntoIter {
    let Writer{ run_writer: (a, _w)} = self;
    WriterIterator{
      traversed: false,
      value: a.clone()
    }
  }
}

impl<A, W: Monoid, F> Writer<(A, F), W> 
    where F: FnOnce(W) -> W {

     fn pass(self) -> Writer<A, W> {
        let Writer{ run_writer: ((a, f), w)} = self;
        Writer{ run_writer: (a, f(w))}
     }
}


impl<A, W: Monoid + Clone> Writer<A, W> {

   pub fn bind<B, F>(self, f: F) -> Writer<B,W>
        where 
          F: Fn(A) -> Writer<B,W>,
          Self: Sized,
     {
        let Writer{ run_writer: (a, w)} = self;
        let (a1, mut w1) = f( a).run_writer ;
        Writer{ run_writer: (a1, w.mappend(&mut w1))}
     }

    pub fn pure(x: A) -> Self {
        Writer{ run_writer: (x, W::mempty())}
    }
    
    
    pub fn lift<B>(self, x: B) -> Writer<B, W> {
        let Writer{ run_writer: (_, w)} = self;
        Writer{ run_writer: (x, w)}
    }

    pub fn unwrap_pair(self) -> (A, W) {
        let Writer{ run_writer: (a, w)} = self;
        (a, w)
    }
    
    pub fn unwrap(self) -> A {
        let Writer{ run_writer: (a, _)} = self;
        a
    }
  
    pub fn listen<>(self) -> Writer<(A, W), W> {
        let Writer{ run_writer: (a, w)} = self;
        Writer{ run_writer: ((a, (&w).clone()), w)}
    }

    pub fn listens<T, F: Fn(W) -> T>( self, f: F) -> Writer<(A, T), W> {
        let Writer{ run_writer: (a, w)} = self;
        Writer{ run_writer: ((a, f( (&w).clone())), w)}
    }
    
    pub fn censor<F: FnOnce(W) -> W>(self, f: F) -> Writer<A, W> {
        let Writer{ run_writer: (a, w)} = self;
        Writer{ run_writer: ((a,f), w)}.pass()
     }
}

pub fn pure_log_string<A>(x: A) -> Writer<A, String> {
    Writer{ run_writer: (x, String::mempty())}
}

pub fn tell<W>(s: W) -> Writer<(), W> {
        Writer{ run_writer: ((), s)}
    }
    
pub fn tell_string(s: String) -> Writer<(), String> {
        Writer{ run_writer: ((), s)}
    }

pub fn tell_str(s: &str) -> Writer<(), String> {
        Writer{ run_writer: ((), String::from( s))}
    }

pub fn tell_array<T: Clone>(v: &[T]) -> Writer<(), Vec<T>> {
        Writer{ run_writer: ((), Vec::from( v))}
    }

pub fn tell_vec<T: Clone>(v: Vec<T>) -> Writer<(), Vec<T>> {
        Writer{ run_writer: ((), v)}
    }
    
/// Macro for a [Writer monad](https://wiki.haskell.org/All_About_Monads#The_Writer_monad)
///
/// The logger type (a local *Monoid* instance) can be established by using a `tell_...()` generator
/// or by constraining the type of the macro result.
#[macro_export]
macro_rules! wrdo {
  (pure $e:expr                           ) => [Writer::pure($e)];
  (let $v:ident = $e:expr ; $($rest:tt)*) => [Writer::pure($e).bind( move |$v| { wrdo!($($rest)*)} )];
  (_ <- $monad:expr ; $($rest:tt)* ) => [($monad).bind( move |_| { wrdo!($($rest)*)} )];
  (&$v:ident <- $monad:expr ; $($rest:tt)* ) => [($monad).bind( move |&$v| { wrdo!($($rest)*)} )];
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [($monad).bind( move |$v| { wrdo!($($rest)*)} )];
  ($monad:expr                            ) => [$monad];
}



