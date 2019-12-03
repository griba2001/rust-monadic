use monadic::{wrdo, writer::{Writer, tell, censor_do}};
use monadic::util::concat_vec_array;
use partial_application::partial;


fn main() {

    let res : Writer< _, Vec<_>> = wrdo!{ 
    
        _ <- tell( vec![1,2,3]) ;
        censor_do( partial!( concat_vec_array => _, &[4,5,6]),
                   wrdo!{
                        x <-  pure 1 ;
                        let z = x+1;
                        pure (x, z)
                    })
        }.listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
