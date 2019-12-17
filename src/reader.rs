// mod reader

pub struct Reader<'a, E, A> { 
  pub run_reader: Box< dyn 'a + Fn(E) -> A>, 
}

impl<'a, E: 'a + Clone, A: 'a + Clone> Reader<'a, E, A> {

  pub fn pure(x: A) -> Self {
    Reader { run_reader: Box::new( move |_| x.clone())}  // (e -> a)
  }
  
     
  pub fn initial_env(self, e: E) -> A {
       (* self.run_reader) (e)
  }
  
  pub fn bind<B, F>(self, f: F) -> Reader<'a, E, B>
        where 
          F: 'a + Fn(A) -> Reader<'a, E, B>,
          B: 'a,
     {
       Reader { run_reader: 
           Box::new( move |e: E| { (* f( (* self.run_reader)( e.clone()) ).run_reader)( e) })
       }
     }
     
  pub fn local<F: 'a>(self, f: F) -> Reader<'a, E, A>
     where
       F: Fn(E) -> E,
  {

    Reader { run_reader: 
           Box::new(move |e: E| { (*self.run_reader) (f(e)) })
        }
  }

}


pub fn ask<'a, E: Clone>() -> Reader<'a, E, E> {

  Reader { run_reader: Box::new(|e: E| e.clone())}
}

pub fn local<'a, E, A, F>(f: F, rdr: Reader<'a, E, A>) -> Reader<'a, E, A>
     where
       F: 'a + Fn(E) -> E,
       E: 'a + Clone, 
       A: 'a + Clone,
  {

    Reader { run_reader: 
           Box::new(move |e: E| { (*rdr.run_reader) (f(e)) })
        }
  }

  
/// macro for a `Reader<'a, E, A>` monad with a boxed `(env -> a)` function  
#[macro_export]
macro_rules! rdrdo {
  // (rdrdo! $body:block) => [rdrdo!$body];
  (pure $e:expr                           ) => [Reader::pure($e)];
  (let $v:ident = $e:expr ; $($rest:tt)*) => [Reader::pure($e).bind( move |$v| { rdrdo!($($rest)*)} )];
  (_ <- $monad:expr ; $($rest:tt)* ) => [Reader::bind(($monad), move |_| { rdrdo!($($rest)*)} )];
  ($v:ident <- pure $e:expr ; $($rest:tt)* ) => [Reader::bind( Reader::pure($e), move |$v| { rdrdo!($($rest)*)} )];
  ($v:ident <- $monad:expr ; $($rest:tt)* ) => [Reader::bind(($monad), move |$v| { rdrdo!($($rest)*)} )];
  ($monad:expr                            ) => [$monad];
}
