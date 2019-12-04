//! examples/writer2.rs
//! 
//! you may set the logger type 
//! by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type 
//! where String is the default if omitted
//! as in `let res : Writer< _, Vec<_> > = wrdo!{...}`
//!
//! `censor(), listen() and listens()` can be used as functions or as methods of a Writer bloc

use monadic::{wrdo, writer::{Writer, tell, censor}};
use monadic::util::concat_vec_array;
use partial_application::partial;


fn main() {

    let modify_log = partial!( concat_vec_array => _, &[4,5,6]);
    
    let res : Writer< _, Vec<_>> = wrdo!{ 
    
        _ <- tell( vec![1,2,3]) ;
        
        // run a subbloc with a modified environment
        censor( modify_log,
                   wrdo!{
                        x <-  pure 1 ;
                        let z = x+1;
                        pure (x, z)
                    })
        }.listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
