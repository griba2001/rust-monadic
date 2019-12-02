use monadic::{wrdo, writer::{Writer, tell}};
use monadic::util::concat_vec_array;
use partial_application::partial;


fn main() {

    let res = wrdo!{ 
    
        _ <- tell( vec![1,2,3]) ;
        x <- pure 1 ;
        let z = x+1;
        pure (x, z)
        
    }.censor( partial!( concat_vec_array => _, &[4,5,6])
            ).listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
