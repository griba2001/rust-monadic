//! examples/writer2.rs
//! 
//! you may set the logger type 
//! by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type 
//! where String is the default if omitted
//! as in `let res : Writer< _, Vec<_> > = wrdo!{...}`
//!
//! `censor(), listen() and listens()` can be used as functions or as methods of a Writer bloc
#[allow(unused_imports)]
use monadic::{wrdo, writer::{Writer, tell, censor, listen}};
use monadic::util::concat_vec_array;
use partial_application::partial;

type Log = Vec<i32>;

fn main() {

    let modify_log = partial!( concat_vec_array => _, &[4,5,6]);
    
    let res : Writer< _, Log> = wrdo!{ 
    
        _ <- tell( vec![1,2,3]) ;
        
        // run a subbloc and modify the log afterwards
        pair <-censor( modify_log,
                   wrdo!{
                        _ <- tell( vec![0]) ;
                        pure 2
                    }.listen());
        pure pair            
        }.listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
