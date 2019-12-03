// examples/reader1
use monadic::{rdrdo, reader::{Reader, ask}};
use partial_application::partial;
use std::collections::HashMap;

type Env = HashMap<String, i32>;

fn my_ini_env() -> Env {
   let mut dict = HashMap::new() ;
   dict.insert( String::from("a"), 1i32);
   dict
}   

fn immutable_add(k: &str, v: i32, dict: Env) -> Env {
   let mut dict1 = dict.clone();
   dict1.insert( String::from(k), v);
   dict1
}

fn main() {

  let my_env_to_env = partial!(immutable_add => "b", 2, _);
  
  let bloc: Reader<'_, Env, _>  = rdrdo!{
  
       env1 <- ask();
       pair <- rdrdo!{ 
               x <- pure 9;
               y <- ask();
               pure (x, y)
             }.local( my_env_to_env) ;
             
       pure (env1.clone(), pair)      
    };


  let res = bloc.initial_env( my_ini_env() );

  println!("result: {:?}", res);  
}
