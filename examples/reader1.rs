//! examples/reader1
//!
//! You must specify in a type restriction the type of the environment of the Reader bloc
//!
//! `local` can be used as a function or as a method

use monadic::{rdrdo, reader::{Reader, ask, local}};
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
  
  let bloc1: Reader<'_, Env, _>  = rdrdo!{
  
       env1 <- ask();
       
       // run a subbloc with a modified environment
       pair <- local( modify_env, rdrdo!{ 
       
               x <- pure 9;
               y <- ask();
               pure (x, y)
             }) ;
             
       pure (env1.clone(), pair.0, pair.1)      
    };


  let res = bloc1.initial_env( my_initial_env() );

  println!("result: {:?}", res);  
}
