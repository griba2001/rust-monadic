use monadic::monadic;
use num::Integer;

fn main() {
    let xs = monadic!{ 
    
        x <- 1..7;
        y <- 1..x;
        guard (&y).is_odd() ;
        let z = &y + 1 ;
        Some((x, z)) 
        
    }.collect::<Vec<(i32,i32)>>();
    
    println!("result: {:?}", xs); 
}
