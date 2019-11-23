use monadic::{mdo, monad::{Bind, Monad}};
use num::Integer;

fn main() {
    let xs = mdo!{ 
    
        &x <- &vec![1,2,3,4];  // with item refs (&x) in the lambda argument position
        guard x.is_odd() ;
        let z = x + 1 ;
        Option::pure((x, z)) 
        
    }.collect::<Vec<(i32,i32)>>();
    
    println!("result: {:?}", xs); 
}
