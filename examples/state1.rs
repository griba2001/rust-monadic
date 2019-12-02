use monadic::{stdo, state::{State, get, put}};

fn main() {
  let res = stdo!{
  
       x <- pure 9;
       y <- get();
       _ <- put( 1);
       z <- get(); 
       pure (x, y, z) 
       
    }.initial_state( 0);

  println!("result: {:?}", res);  
}
