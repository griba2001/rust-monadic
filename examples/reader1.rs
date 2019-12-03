// examples/reader1
use monadic::{rdrdo, reader::{Reader, ask, local_do}};
use partial_application::partial;
use std::collections::HashMap;

type Env = HashMap<String, i32>;

fn immutable_add( k_slice: &str, v: i32, dict: Env) -> Env {
   let mut dict1 = dict.clone();
   dict1.insert( String::from(k_slice), v);
   dict1
}

fn my_initial_env() -> Env {
   immutable_add( "a", 1, HashMap::new())
}   


fn main() {

  let my_env_to_env = partial!(immutable_add => "b", 2, _);
  
  let bloc: Reader<'_, Env, _>  = rdrdo!{
  
       env1 <- ask();
       pair <- local_do( my_env_to_env, rdrdo!{
       
               x <- pure 9;
               y <- ask();
               pure (x, y)
             }) ;
       pure (env1.clone(), pair)      
    };


  let res = bloc.initial_env( my_initial_env() );

  println!("result: {:?}", res);  
}
