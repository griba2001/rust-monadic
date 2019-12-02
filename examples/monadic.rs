use monadic::monadic;
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
        w <- pure 5;     // (<-) rhs pure     
        pure (x, z, w)
        
    }.collect::<Vec<_>>();
    
    println!("result: {:?}", xs); 
}
