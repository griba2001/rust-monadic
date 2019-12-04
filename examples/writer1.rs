//! examples/writer1.rs
//!
//! you may set the logger type 
//! by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type 
//! where String is the default if omitted
//! as in `let res : Writer< _, String > = wrdo!{...}`
//!
//! `censor(), listen() and listens()` can be used as functions or as methods of a Writer bloc

#[allow(unused_imports)]
use monadic::{wrdo, writer::{Writer, tell, tell_str, censor}};
use monadic::util::concat_string_str;
use partial_application::partial;

fn main() {
    
    let modify_log = partial!( concat_string_str => _, "log2");
    
    let res : Writer< _, String> = wrdo!{ 
    
        _ <- tell_str( "log1") ;
        
        // run a subbloc with a modified log
        censor( modify_log,
                   wrdo!{
                        x <-  pure 1 ;
                        let z = x+1;
                        pure (x, z)
                    })
        }.listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
