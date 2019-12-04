//! examples/state1.rs
//!
//! You may specify in a type restriction the type of the State bloc
//! or apply it directly to an initial_state without the type restriction

use monadic::{stdo, state::{State, get, put}};

fn main() {
  let bloc: State<'_, i32, _> = stdo!{
  
       x <- pure 9;
       y <- get();
       _ <- put( 1);
       z <- get(); 
       pure (x, y, z) 
       
    };
    
  let res = bloc.initial_state(0);  

  println!("result: {:?}", res);  
}
