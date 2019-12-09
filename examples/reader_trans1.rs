// examples/reader_trans1

#[allow(unused_imports)]
use monadic::{rdrt_mdo, monad::{Monad}, 
              reader_trans::{ReaderT, lift, ask, local}};
use num::Integer;
use partial_application::partial;
use std::collections::HashMap;

type Env = HashMap<String, i32>;

fn immutable_insert( k_slice: &str, v: i32, dict: Env) -> Env {
   let mut dict1 = dict.clone();
   dict1.insert( String::from(k_slice), v);
   dict1
}

fn my_initial_env() -> Env {
   immutable_insert( "a", 1, HashMap::new())
}   

fn main() {
  let modify_env = partial!(immutable_insert => "b", 2, _);

  // example with Vec as the nested monad
  
  let bloc = rdrt_mdo!{   // possible type restriction as ReaderT<'_, Env, Vec<_>>
  
       env1 <- ask() as ReaderT<'_, Env, Vec<Env>>;
       pair <- local( modify_env, rdrt_mdo!{
       
               x <- lift (5..9).collect::<Vec<i32>>();
               guard x.is_odd();
               
               y <- ask() as ReaderT<'_, Env, Vec<Env>>;
               
               // this acts as a typed `pure` specifying the monad type
               lift Vec::pure((x, y))   
             }) ;
             
       // reader type restriction unnecessary ending with lift instead of pure
       lift Vec::pure((env1.clone(), pair.0, pair.1))      
    };

  // applying the initial_env() to the contained (env -> m a) 
  // returns the nested monad structure
  
  let res = bloc.initial_env( my_initial_env() );

  println!("result: {:?}", res);  
}
