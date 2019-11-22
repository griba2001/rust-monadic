use monadic::{monadic, Monad};
use num::Integer;

fn main() {
    let xs = monadic!{ 
    
        x <- 1..7;
        y <- 1..x;
        guard (&y).is_odd() ;
        let z = match x.is_even() { 
                    true => &y + 1,
                    _ => &y - 1,
                };
        Option::pure((x, z)) 
        
    }.collect::<Vec<(i32,i32)>>();
    
    println!("result: {:?}", xs); 
}
