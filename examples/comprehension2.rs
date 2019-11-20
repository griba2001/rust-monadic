use monadic::{monadic, Monad};
use num::Integer;

fn main() {
    let xs = monadic!{ 
    
        &x <- &vec![1,2,3,4];
        guard x.is_odd() ;
        let z = x + 1 ;
        Option::pure((*x, z)) 
        
    }.collect::<Vec<(i32,i32)>>();
    
    println!("result: {:?}", xs); 
}
