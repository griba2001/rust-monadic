//! you may set the logger type 
//! by beginning with a `tell...` function within the macro `wrdo` 
//! or by declaring it as the result type 
//! where String is the default if omitted
//! as in `let res : Writer< _, String > = wrdo!{...}`

#![allow(unused_imports)]

use monadic::{wrdo, writer::{Writer, tell, tell_str, censor_do}};
use monadic::util::concat_string_str;
use partial_application::partial;

fn main() {
    
    let res : Writer< _, String> = wrdo!{ 
    
        _ <- tell_str( "log1") ;
        censor_do( partial!( concat_string_str => _, "log2"),
                   wrdo!{
                        x <-  pure 1 ;
                        let z = x+1;
                        pure (x, z)
                    })
        }.listen() ;
    
    println!("result: {:?}", res.unwrap()); 
}
