use monadic::{stdo, state::{State, get, put}};

fn main() {
  let res = stdo!{
       x <- get();
       _ <- put( 1);
       y <- get(); 
       pure (x, y) 
    }.initial_state( 0);

  println!("result: {:?}", res);  
}
