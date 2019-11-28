use monadic::{wrdo, writer::{Writer, tell_vec}};
use monadic::util::concat_vec_array;
use partial_application::partial;


fn main() {

    let res = wrdo!{ 
        _ <- tell_vec( vec![1,2,3]) ;
        x <-  Writer::pure(1) ;
        let z = x+1;
        pure (x, z)
    }.censor( partial!( concat_vec_array => _, &[4,5,6])
            ).listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
